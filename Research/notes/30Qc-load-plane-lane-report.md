# 30Qc — `lane-load-plane-precision`: the seat map, and the two execute halves

> Tier: builder lane report (Opus, MAP half; branch `ai/r30-lane-load`, worktree
> `.claude/worktrees/agent-aa4d08a95909cb524`, base `aabcc2d9`). Grades: +SURE / ~SUSPECT /
> -GUESS / --WONDER. Every line number is MEASURED at `aabcc2d9` unless it names a commit of
> this lane. Charters that outrank this note: `plans/30P` (`the-load-principles`,
> `the-emission-planner`, `the-stream-forms`), `plans/30I` §3 + §14, `spike/CLAUDE.md`,
> `crates/{syntax,analysis,oracle,cli}/CLAUDE.md`.
>
> §map is this section — the proposal EXECUTE-A and EXECUTE-B implement. EXECUTE-A and
> EXECUTE-B append their own sections; neither edits this one.

## §map

### the-model-in-one-screen

Three answers, and every seat below is one of them.

1. **A load head is EXACT or a point havoc, nothing between**
   (`30P:rul-load-head-is-exact-or-havoc`). No POSSIBLE state, no engine-selected snapshot
   singleton, no runtime-verified candidate — all three are STRUCK. EXACT has exactly one
   witness: authored text the controller can evaluate over controller-held inputs (a literal,
   a book-set root, `$0` under `30P:model-symbolic-dollar-zero`, or a statically-evaluable
   stdlib predict, which is not this lane's).
2. **An unresolvable `.` is a POINT havoc across FIVE domains**
   (`30P:principle-unknown-source-is-a-point-havoc`), not a suffix poison and not a whole-unit
   one. Function bindings recover by sh's last-wins; cwd, shell options, positionals, and
   termination each get their own answer.
3. **Acquiring bytes is not modelling them.** A plain-sh `.` is READ, recorded as a load
   occurrence, and mirrored beside the plan — and analyzed NOT AT ALL
   (`FORFEITS:forfeit-plain-sh-inclusion-analysis`). The site still walls.

Two laws bound every seat and neither may be violated by any commit below.
`30P:rul-guard-resolves-like-its-mutation` [TYPED 2026-08-22, not a proposal]: Dorc never
interprets or compares what a vouch checks; it only ensures its OWN movement or renaming never
creates a binding combination ordinary shell resolution could not have produced.
`spike/CLAUDE.md rul-unsure-falls-toward-sh-parity`: load order, name resolution, scoping and
binding fall toward GENERIC sh parity, never toward what this round happens to need.

### fnd-measured-today — what the code actually does

- **`fnd-top-is-absorbing-not-pointwise`** (+SURE). `analysis/src/funcenv.rs:2116-2124`
  (`command_transfer`'s `"." | "source"` arm) returns `EnvStack::Top` when the operand cannot be
  read or `program_of_dot_target` is `None`. `EnvStack::Top` is the LATTICE top of the whole
  stack: `bind` (`:520-527`) matches only `Frames`, `push`/`pop` (`:529-550`) pass `Top`
  through, and `join` (`:562-`) absorbs. So the poison never recovers along that flow path, and
  a later unconditional `FuncDef` — whose transfer is `Merge` calling `env.bind` at `:2016-2022`
  — is inert. That is the whole of `p-x-unknown-source-is-a-point-havoc`.
- **`fnd-four-domains-have-no-seat`** (+SURE by grep and by read). Only the FUNCTION-BINDING
  domain exists. `DefinitionTable::cwd` (`funcenv.rs:188-192`) is ONE modelled cwd for the whole
  unit, with no program point — "cwd ⊤ after this line" is not representable in the type.
  Shell options live in `analysis/src/cfg.rs`'s forward errexit pass (`materialise_errexit_edges`
  `:1640`, `errexit_after` `:1826`, `errexit_toggle` `:1842`), which walks straight through a
  `.`. Positionals below a book's top level read unresolvable already (`variable_text` answers
  `None`) — but for the DULLER reason that nothing binds them, and the raw lattice value is ⊥,
  not ⊤ (`fnd-never-assigned-variable-reads-bottom`, below). Termination is May-reach already,
  by the CFG simply not adding an exit edge.
- **`fnd-never-assigned-variable-reads-bottom`** (+SURE, measured while authoring the positional
  cell). `ValueFlow::variable_before(node, "1")` answers `Flat::Bottom`, not `Flat::Top`, at a
  REACHED node. ⊥ is supposed to mean "unreached"; here it means "nothing recorded". The two are
  safe only because `SourceLiteralPlane::variable_text` (`funcenv.rs:119-127`) maps both to
  `None`. Not this lane's to repair — recorded so nobody reads ⊥ as "unreached" in the value
  plane, and so the cell asserts at the consumer seat rather than on the raw value.
- **`fnd-param-operator-dies-in-the-lexer`** (+SURE). `${0%/*}` never reaches the AST as anything
  decodable: `lexer.rs:668-702` (`lex_braced_param`) grabs the whole `${…}` body, tests it for
  name-chars only, and on failure mints `LexPart::ParamComplex { empty_defaulted }` — the body
  BYTES are dropped. `empty_defaulted` (`:705-716`) recovers a name for `${x-}`/`${x:-}` alone.
  `ast.rs:230-245` documents it as deliberately opaque. So the operator and pattern are gone
  before any analysis; the operand is a non-literal word and `admits_a_load` refuses it.
- **`fnd-no-dollar-zero-anywhere`** (+SURE by absence). Nothing binds `$0`. The lexer maps it to
  `WordPart::Param { name: "0" }` (`lexer.rs:649-653`, the digit branch), and it resolves as an
  ordinary never-assigned variable. The book path IS already on the snapshot
  (`snapshot.rs:243` `book_path()`), threaded from `read_book_sourced`'s caller — so the fact
  the model needs is present at the edge and simply never reaches the load plane.
- **`fnd-computed-dot-is-parse-tier`** (+SURE). `parser.rs:1075-1093` refuses a `.`/`source`
  operand carrying `CommandSubst`/`Arithmetic` (`word_has_expansion_effect`, `:818-827` — it
  does NOT flag `Param`/`ParamComplex`), minting `Unsupported` at `Severity::Error`;
  `main.rs:1037-1041` folds any parse-or-CFG Error into one whole-run `book_unmodeled` boolean
  and exit 10. The precedent to copy is `wrapper_incoherent` (`main.rs:960-961`, ranked at
  `:1042-1049`, `EXIT_WRAPPER_INCOHERENT = 11`); exit codes 10..=16 are taken, so a new one is 17.
- **`fnd-plain-sh-dies-on-the-host`** (+SURE, MEASURED this lane, upgrading `30O`'s ~SUSPECT).
  `cli/src/sourcing.rs:129-131` gates acquisition on the dorc-lang marker AND a clean
  `load_inert` lint; `main.rs:478-483` reads the bytes and THROWS THEM AWAY on failure. The new
  case `load30-plain-sh-inclusion-ships`, run with its `XFAIL` marker temporarily removed, fails
  with exactly one failure: *"the published plan sources `./helpers.sh`, which the generation
  does not contain"*. So the generated plan really does carry a dangling `.`, which the atlas
  measured FATAL (`floor30-atlas-dot-missing-file-is-fatal`). The same gate is what makes
  `load31-punted-load-shapes` red, which means **`load31`'s `expected.ran` is not compared
  today** — `run_round_trip` skips `exec_check` when `run.failures` is non-empty
  (`tests/e2e.rs:1793`), and the XFAIL lens then returns `Ok`. Not a defect; worth knowing
  before anyone reads that file as an assertion.
- **`fnd-bundle-projection-strips-every-source`** (+SURE). `cli/src/bundle.rs:398` runs
  `strip_file_with_map` over EVERY occurrence's source. Correct for dorc-lang; wrong for a
  plain-sh inclusion, which is BOOK-CLASS material and must be mirrored byte-verbatim
  (`spike/CLAUDE.md two-surfaces`, `KNOBS:kBACKFLIPS`). EXECUTE-B must branch here.
- **`fnd-assignment-bearing-dot-is-inlineable`** — `30Pc:bug-assignment-bearing-dot-is-inlineable`
  CONFIRMED at `artifact.rs:366`: `NodeKind::Simple { words, redirs, .. }` discards `assigns`, so
  `MODE=prod . ./entry.oracle.sh` is `absorbable` and `inline_imports` replaces the whole command,
  dropping `MODE=prod`. Repair is EXECUTE-B's first commit (see `30Pc:required-repair`, all five
  steps, unchanged).

### item-1 — point-havoc over the five v0 domains (EXECUTE-A)

**D1, function bindings — the lattice change, and its monotonicity argument.**

Replace the absorbing `EnvStack::Top` at the `.` arm with a POINTWISE havoc: bind
`Flat::Top` for every name in the solve's `universe` into the INNERMOST frame.

```rust
impl EnvStack {
    /// Every name the unit knows becomes unknown HERE and no further
    /// (`30P:principle-unknown-source-is-a-point-havoc`). Pointwise, in the innermost frame,
    /// because that is where a write lands and where a later binding overwrites it.
    fn havoc_names(&mut self, universe: &BTreeSet<String>) { … }
}
```

`transfer` already holds `universe`; thread it into `command_transfer`. Four riders:

- **Monotone** (+SURE): `havoc(x)` is `x` with a fixed set of keys raised to ⊤ in one frame.
  For `x ⊑ y` both `Frames` of equal depth, raising the same keys in both preserves the
  pointwise order; `x = Frames(_) , y = Top` gives `Frames(…) ⊑ Top`; unequal depths are
  incomparable (`join` answers `Top`), so the obligation is vacuous. Finite height is
  unchanged — no new lattice values exist.
- **`EnvStack::Top` STAYS** for what it is honestly for: an unparsed `CfgNodeKind::Top` node, a
  join across unequal frame depths, and a definition statement the table does not know
  (`definition_at`, `funcenv.rs:2098`). Do NOT reuse the pointwise havoc for those — a `Top`
  node may have pushed or popped a scope, so its stack SHAPE is unknown too.
- **Subshell scope**: because the havoc lands in the innermost frame, `ScopeExit`'s `pop`
  discards it. That is sh: a `.` inside `( … )` binds nothing outside. It IS a precision gain
  over today (where `Top` survives the pop), so it is a licensing widening and must be reported
  as one. It is the sh-parity answer, and `floor30-atlas-subshell-nesting-and-removal-scope`
  is the measurement it rests on.
- **Names OUTSIDE the universe are untouched** and keep answering `NoOpinion`
  (`LiveDefinitions::definition_before`, `funcenv.rs:771-773`). That is deliberate and is the
  human-adopted reading: an unknown file redefining a TOOL as a function is the same cell as the
  host's PATH resolving that tool to anything, which `rul-guard-resolves-like-its-mutation`
  places on the admin's side of the horizon. Do NOT "fix" it by giving frames a ⊤ default — that
  is the reviewer clause the human NACKED, and it would hollow out the whole item.

Anti-regression, both already GREEN and both must stay: `a_name_bound_only_before_an_unknown_source_is_withheld_after_it`
and `an_unknown_source_keeps_defensive_emission_and_its_wall` (`cli/tests/sh_parity.rs`).
`unprovable()` (`funcenv.rs:1241-1247`) reads `lookup(name) == Flat::Top` and keeps working
unchanged — a havoc'd-and-not-rebound name is still ⊤ there.

**D2, cwd.** New, and it is what stops D1 from opening a hole: with bindings recovering, a
later relative `.` would resolve and BIND off a cwd the unknown source may have moved.

Representation, deliberately NOT a lattice domain: a pre-pass set. `load_sites`
(`funcenv.rs:1926-1962`) already runs before the solve, from the value plane alone. Extend it to
compute `cwd_clobbers` = {a `.` site whose OPERAND could not be evaluated} ∪ {a `.` site whose
target is an `Included` plain-sh file} ∪ {any `cd` command node}, then take the forward CFG
reachability closure. At a node in that closure a RELATIVE operand answers `Havoc(CwdUnknown)`.

- Why not the `named` bucket: a book-sourced dorc-lang dependency is `named`+`unresolvable` in
  acquisition round 1 and only becomes resolvable once read. Seeding clobbers from `named` would
  make the acquisition fixpoint unable to grow. Clobbers are OPERAND-unevaluable sites only.
- Why `cd` joins: today a book that `cd`s before a `.` resolves that `.` against the ORIGINAL
  cwd (`30I` §3.2 names the gap: "full book cwd flow is still owed"). It is the same hole from
  another door and it is cheap to close in the same predicate. Corpus risk measured: the only
  `cd`s in the corpus are in floor atlas manifests and two `why-*` looms, none above a `.`.
- Direction: strictly withholding. Fewer resolutions, fewer bindings, fewer licenses.

**D3, shell options.** `cfg.rs` already has the vocabulary — `ErrExit` is a three-value lattice
and `may_be_on()` covers `On` and `Top`, which is how `set "$dyn"` is handled. The change is one
arm: a `.`/`source` node whose target is not a closed load-inert program sets errexit to
`ErrExit::Top` in `errexit_after`. Direction is safe by construction: a failure→exit edge is
ADDED beside the fall-through, never in place of it, so the edge set is a superset of both worlds
and nothing downstream gains reach.

**D4, positionals — already unreadable; say where.** `variable_text` answers `None` for `$1` at a
book's top level. Pinned GREEN so a later precision pass cannot turn "we never modelled this"
into "we know it did not change". NB `fnd-never-assigned-variable-reads-bottom`.

**D5, termination — already May-reach; say where.** The CFG adds no exit edge at a `.`, so the
line below is modelled as running, which is the safe direction for elision AND for guards.
Pinned GREEN. Do not "refine" it into a maybe-unreachable: that would license removing lines.

**Rule for the whole item — `rul-included-is-as-opaque-as-unresolvable`**: an `Included`
plain-sh target (item 7) is exactly as opaque as an unresolvable one for D1–D5. The only thing
acquisition buys is bytes to ship. State it once in the code; do not let it become five
per-domain special cases.

### item-2 — the parameter-expansion decode and the symbolic `$0` (EXECUTE-A)

**The AST change, and its ⊤-reject boundary.** Reshape the opaque part in place
(`rul-strawman-formats-no-compat` — rename/reshape, never an adapter):

```rust
/// A parameter expansion carrying an operator. The BASE and the OPERATOR are decoded; the
/// operand word is lexed with the ordinary word machinery, so quoting inside it is honest.
ParamExpansion { base: String, op: ParamOp },

enum ParamOp {
    /// `${x-}` / `${x:-}` — the closed nounset-safe form, whose default is EMPTY.
    EmptyDefault { colon: bool },
    /// `${x-w}` `${x:-w}` `${x+w}` `${x:+w}` `${x=w}` `${x:=w}` `${x?w}` `${x:?w}`.
    Substitute { kind: SubstituteKind, colon: bool, word: Vec<WordPart> },
    /// `${x%w}` `${x%%w}` `${x#w}` `${x##w}` — the four trims, pattern as a lexed word.
    Trim { end: TrimEnd, greedy: bool, pattern: Vec<WordPart> },
    /// `${#x}`.
    Length,
    /// Everything else, INCLUDING the bash-family forms the dialect bans.
    Unmodelled,
}
```

- **Which operators, argued.** `%`/`%%`/`#`/`##` are load-bearing (`${0%/*}` is the pin;
  `${0##*/}` is the basename idiom and is equally common). `-`/`:-` must be decoded because
  `empty_defaulted` is LOAD-BEARING TODAY — `funcenv`'s `LoadCondition::Value` sentinel path
  reads it (`30I` §2.2) — so keep a projection `ParamOp::default_word_is_empty()` and make that
  path a rename, not a redesign. The rest are decoded because the operator lexer costs nothing
  once it exists, and a decoded-but-unmodelled operator is a better diagnostic than an opaque
  word.
- **The ⊤-reject boundary** (`inv-top-reject`, `syntax/CLAUDE.md top-reject-here`): decoding is a
  SYNTAX act and mints no diagnostic. The load-plane EVALUATOR is where ⊤ lives — `Unmodelled`,
  `Length`, `Substitute` over a non-controller-known base, and any pattern part that is not
  `Literal`/`SingleQuoted`/`DoubleQuoted(literals)` all answer ⊤. That is
  `semantic-top-not-here` observed: the parser preserves losslessly, the analyzer decides.
- **Do NOT re-lex the body from a captured string.** Decode in place inside `lex_braced_param`,
  which is already positioned in `self.src`, so spans stay real. Re-lexing is what
  `syntax/CLAUDE.md tn-coarse-subst-provenance` already regrets for command substitution.

**The `$0` model.** `$0` is NOT a variable and must not be seeded into `ValueFlow` — a
single-valued plane cannot carry two spellings, and seeding one would silently pick it.
It is a controller-held fact, so it rides `DefinitionTable` beside `cwd`, for the reason that
type already carries `cwd`: the load answer and the definitions it binds must be ONE fact.

```rust
/// The authored book path `$0` names, and the invocation spellings the analysis must hold for
/// (`30P:model-symbolic-dollar-zero`). Never realpath'd — sh-parity under symlinks — and never
/// read from a shell.
pub struct ScriptSpellings { … }
pub enum Spelling { SlashBearing, Slashless }
```

- **`SlashBearing`** = the book path as given if it contains a `/`, else `./` + basename. This is
  the spelling Dorc itself invokes (`30P:rul-dorc-invokes-in-a-modelled-live-spelling`:
  `sh ./plan.sh` from the generation root).
- **`Slashless`** = the bare basename, LIVE only when the book's own directory is the modelled
  load cwd — otherwise `sh <basename>` could not have found the book and the spelling is not a
  possible invocation. Both spellings evaluate against the SAME modelled cwd; the spelling varies
  `$0`'s string and nothing else.
- Minted at ONE seat, `cli::world::definition_table` (`world.rs:760`), from
  `snapshot.book_path()` + `snapshot.cwd()`. `DefinitionTable::rooted_at` grows the parameter, so
  a table with a cwd and no `$0` is unrepresentable.

**The operand evaluator.** One function, composed so nothing regresses:

```
resolve_operand(node, index, spelling):
    if let Some(text) = literals.literal_text(node, index) { return Resolves(text) }  // today's answer
    evaluate_word(ast_word_at(node, index), spelling)                                  // the new tier
```

Delegating to `literal_text` FIRST preserves the positional overlay and constant folding that
`argv_values` already does, so spliced bodies and `$1` in an operand behave exactly as today.
`evaluate_word` walks `WordPart`s: `Literal`/`SingleQuoted` → text; `DoubleQuoted` → recurse;
`Param { name: "0" }` → the spelling's `$0`; any other `Param` → **`SourceLiteralPlane::variable_text`,
never `ValueFlow` directly**; `ParamExpansion` → base then operator; `CommandSubst`/`Arithmetic`
→ ⊤ with the cause recorded.

> **The wall stays standing.** `funcenv-reads-source-literal-plane-only` (`28K` §2) survives
> because the evaluator reads the AST only for STRUCTURE and routes every VARIABLE read through
> the grade-gated plane. `$0` is not a variable read — it is a controller-held constant. Add a
> lexical fence in `funcenv`'s own test module asserting the evaluator names no `ValueFlow`
> accessor; that is the same shape as `this_module_names_no_fixpoint_reachable_type`.

**The per-spelling answer, and the EXACT predicate.**

```rust
enum OperandAnswer {
    /// Names this canonical key in the snapshot.
    Resolves(String),
    /// Provably fatal: a NON-FINAL component of the resolved key is a snapshot FILE
    /// (`${0%/*}` of a slashless `$0` gives `book.sh/helpers.sh`). Narrow on purpose.
    Dead,
    /// The HOST picks: a slashless operand (a PATH search — `30P`), or a relative operand at a
    /// cwd-unknown node.
    HostChosen,
    /// The word could not be evaluated over controller-known inputs.
    Unevaluable(HavocCause),
}
```

`HavocCause` ∈ {SpellingsDisagree, DynamicValue, ComputedSubstitution, UnmodelledOperator,
NotInSnapshot}. It is an AID TYPE and it is the whole reason the hints can be specific; mint it
now rather than reconstructing a reason at the diagnostic seat.

EXACT iff every live spelling answers `Resolves(k)` for ONE k, plus the resolution of
`tc-dollar-zero-spelling-asymmetry` below for what a NON-resolving spelling contributes.
Traps measured and already in the corpus's own atlas: `${0%/*}` of a no-slash word is the WHOLE
word; of a book at `/` it is the EMPTY string, never "cwd".

**`tc-dollar-zero-spelling-asymmetry` — FLAGGED, not resolved.** `30P` says "EXACT iff every
live spelling resolves to one snapshot file". Read strictly, a `Dead` or `HostChosen` spelling
denies EXACT and every `${0%/*}` book becomes a havoc — including
`load30-point-havoc-and-script-relative`, which `30P:scheduling-truth` expects to PASS before
the end of r30. Read as `rul-dead-spelling-is-not-unsound` intends, a spelling under which the
`.` cannot succeed is DEAD and earns a lint, not a refusal.

The two readings differ on exactly one cell, and the cwd domain forces the question: with an
unknown `.` above, the flagship's slashless spelling is not `Dead` (we cannot prove `book.sh` is
not a directory under a cwd we do not know) — it is `HostChosen`.

Strawman, so the cell is concrete:

```sh
# under `sh /srv/book.sh`:   ${0%/*} = /srv          ⇒ /srv/hork.dorc.sh
# under `cd /srv && sh book.sh`: ${0%/*} = book.sh   ⇒ book.sh/hork.dorc.sh ⇒ the `.` fails
. "${SITE_PROFILE:-/dev/null}"
. "${0%/*}/hork.dorc.sh"
hork tune web
```

My lean, for the conductor to rule: **the second spelling is a LINT spelling, not a resolving
one.** Dorc invokes S1 and ships a plan whose `$0` it chooses; the elision decision is baked into
the shipped bytes before any `.` runs, so a human re-invoking the artifact under a different
spelling is the same cell as a host whose PATH resolves a tool differently —
`rul-guard-resolves-like-its-mutation`'s own horizon. Under that reading S2 contributes exactly
two things: a `Dead` spelling mints the off-ramp lint, and a spelling that `Resolves` a
DIFFERENT snapshot file denies EXACT (cheap, and a genuine authorship hazard). Everything else
about S2 is advisory. **EXECUTE-A must build the evaluator so the predicate is one function of
the per-spelling answers** — the quantifier is a two-line change either way, so this ruling does
not gate the build.

**The two DiagCodes, minted with `message: None`** (`error-authorship-tier`; builders author ZERO
prose). Loom skeletons go at `crates/aid/tests/<slug>.loom` and publish with
`dorc-loom publish <case>`:

- `script-relative-load-dies-slashless` — when-fires: *a book's `.` operand is computed from `$0`
  by parameter expansion in a way that cannot succeed under a slashless invocation
  (`${0%/*}` of a no-slash `$0` is the whole word), so the load is EXACT for the spelling Dorc
  invokes and DEAD for the other.* why: `30P:rul-dead-spelling-is-not-unsound` — dead is not
  unsound (nothing below a failed `.` runs), so this is an OFF-RAMP lint and never a refusal;
  Dorc must never TEACH the spelling that breaks (`KNOBS:kLANG` stewardship).
- `slashless-source-searches-path` — when-fires: *a `.` operand carries no `/`, so POSIX makes it
  a PATH search rather than a cwd lookup.* why: `30P`'s slashless paragraph — PATH is a host
  read, so the operand is unresolvable by the controller-known axis; the repair is `./x.sh`.

Both are `Warning`-tier hints. Neither is asserted by a red cell committed in this MAP half, and
that is a deliberate deviation — see `dev-lint-cells-wait-for-their-codes`.

### item-3 — the computed `.` moves from parse tier to post-analysis (EXECUTE-A)

Three seats, in order.

1. **`syntax/src/parser.rs:1075-1093`** — the `.`/`source` arm stops consulting
   `word_has_expansion_effect`. The operand parses as an ordinary `Word` with its
   `CommandSubst(AstId)` part intact. `word_has_expansion_effect` itself is UNCHANGED: it still
   serves the for-list trigger and the dynamic-command-name trigger, both of which stay.
   `syntactic-top-triggers` in `syntax/CLAUDE.md` loses one clause and the conductor is owed the
   replacement sentence (see the steering proposals).
2. **`analysis`** — the operand is `Unevaluable(ComputedSubstitution)`, so the site is an
   ordinary point havoc. `load_sites` records the cause so the CLI can tell a computed operand
   from a merely-dynamic one; that distinction is `30P`'s own
   (`p-x-glob-load-no-match-aborts` already turns on the same "evaluated vs unreadable" cut).
3. **`cli/src/main.rs`** — a new post-analysis, pre-network boolean beside `wrapper_incoherent`,
   ranked in the same `if / else if` chain at `:1042-1049`, with `RunOutcome::LoadUnresolvable`
   and `EXIT_LOAD_UNRESOLVABLE = 17` (10..=16 are taken; 17 is the next of the reserved
   dorc-semantic range). A new `DiagCode` with `message: None`; slug strawman
   `computed-source-operand`.

**`tc-computed-dot-complaint-shape` — FLAGGED.** `30P:rul-floor-valid-text-never-parse-fails`
rules "build the necessary MACHINERY, keep the UX narrow — reject-but-stay-agile". The machinery
above is unambiguous. What is the human's: whether the complaint fail-fasts at all, and if so
whether it is per-site or whole-run. Product-surface strawman:

```console
$ dorc plan --book=deploy.sh web1
error[computed-source-operand]: [unwritten: computed-source-operand]
  --> deploy.sh:3:3
   |
 3 | . "$(dirname "$0")/helpers.sh"
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
plan: 6 to run, 0 to verify (0 skipped)
$ echo $?
17
```

My lean: keep today's OUTCOME (a distinct non-zero exit that stops a `dorc … && deploy` chain)
and move only its TIER, because that is the conservative direction and softening it later is a
UX change rather than a kernel rebuild — which is exactly what the ruling asks for.

**PARSER AUTHORIZATION residue** (human lean 2026-08-22: fix the obvious things, redden no test,
list what you saw and did not touch). Seen, not touched, all +SURE:
- `lex_braced_param` (`lexer.rs:668-702`) balances `{}` only, so a `}` inside a quoted pattern
  (`${x%"}"}`) closes the expansion early. The decode above does not fix it; fixing it wants
  quote-aware body scanning.
- `WordPart::CommandSubst` bodies re-lex with inner-relative spans
  (`tn-coarse-subst-provenance`), so a diagnostic inside a computed operand will point at the
  wrong bytes the moment item 3 makes such operands reachable. First real consumer of the
  locator-DAG debt.
- `WordPart::Arithmetic` carries NO body at all — it is a bare marker, so `$(( ))` cannot be
  decoded even where it is trivially controller-known. Deliberate ⊤ today; note it as a
  ⊤-trigger that is unshrinkable rather than merely unshrunk.

### item-4 — the glob order-unknown retarget (LANDED in this MAP half)

`p-x-glob-load-members-are-order-unknown` keeps its `r31:book-load-acceptance` horizon and stays
red. Its assertion and its trigger now name the ruled target
(`30Pb:fnd-glob-order-needs-whole-program-meet`): a UNIVERSAL MEET over every order of the
members' whole load PROGRAMS, not a same-name byte collision. The fixture gained a member that
`unset -f`s a name another member is the SOLE definer of — the two orders disagree, so the meet
withholds a name nothing contests by bytes. Assignments, `cd`, and `exit` inside a member are the
same rule and are NAMED in the doc comment rather than asserted, because they want the deferred
glob lane's whole-program walk and not a third fixture.

### item-5 — the three closure-custody xfails (EXECUTE-B)

All three are ONE shape: `HelperIndex` answers from a flat, file-keyed, site-blind index while
the environment next door already knows better.

- **`p-x-helper-unset-f-across-files`** — `closure.rs:323-331`: the `unset -f` arm removes
  declarations only where `declaration.file == file` (the CURRENT file), and `resolve`
  (`:534-583`) picks `declarations.last()`. Seat: consult
  `funcenv::LiveDefinitions::definition_before` instead of `declarations.last()`. The definition
  table now records non-role funcdefs, so the environment can see the removal
  (`oracle/CLAUDE.md the-frame-lookup-is-the-only-resolution-seat` — this is that rule reaching
  the helper lane).
- **`p-x-regional-helper`** — `closure.rs:266-295`, the book census at `:273-279`, walks the WHOLE
  AST at any depth, so a `FuncDef` inside `( … )` is indistinguishable from a top-level one; the
  consumer at `:547` is purely name-keyed and `closure_for(&self, file, body)` (`:441`) takes no
  site. Seat: `closure_for` gains a site, and the book census becomes region-aware. Interim
  behaviour is already pinned green by `a_book_subshell_helper_suspends_like_an_ambient_one`
  (`oracle/tests/sh_parity.rs:140-174`) — that test states the CURRENT safe-but-coarse answer and
  must be re-pointed, not deleted, when the pin greens.
- **`p-x-definition-grade-keying`** — `cli/tests/sh_parity.rs:802-831`. The keying half landed;
  what remains is that `PredictSet`/`VerdictSet` keep ONE row per `(file, role)`, so the earlier
  of two within-file definitions produces no row for `dorc_core::answering_row` to find. Seat is
  the lift arity in `oracle/src/{predict,verdict}.rs`, NOT `closure.rs`. Its precondition test
  `the_environment_names_a_definition_per_frame_within_one_file` (`:762-783`) is green and proves
  the environment already answers correctly.

Every one of these is `the-frame-lookup-is-the-only-resolution-seat` territory, so
`28Q` §1's ruling binds: with no agreement veto behind it, every function-environment precision
change SELECTS WHOSE JUDGMENT governs a site. License-review-tier; minimal; reported.

### item-6 — guarded-source fidelity (EXECUTE-B)

`funcenv.rs:1618` matches `LoadCondition::Value { name, equals, .. }` and the `..` DISCARDS the
`literal` field (`load.rs:351-359` defines it). `sentinel_decides` (`:1693-1748`) and
`sentinel_arm` (`:1770-1800`) decide STRUCTURALLY — by whether the target closure's declared
names are already bound — and never compare the value. So a package assigning `v1` under a guard
testing `v2` is modelled as REUSED where a real shell sources it again.

The fix is the smaller half of `30I:rul-load-semantics-stay-full-fidelity`: the full load model
retains the live constant, the compared literal, and the branch sh takes. The SEPARATE lossy
speech projection (`30I:rul-guarded-source-speech-is-lossy`) keeps only direct-constant origin,
guarded-source, and helper co-resolution — it must not gain the literal, and it must never become
a substitute load interpreter (`30I` §3.3).

Recover test `176e0818` from branch `worktree-sol-adversarial-30M`
(`git -C $WT show 176e0818`). RECOMMENDATION: **re-author, do not cherry-pick.** It lands in
`funcenv.rs`'s test module as `#[ignore]`d text on a branch whose surrounding lines have moved;
re-authoring it as a `xfail_until("p-x-sentinel-value-conjunct", …)` cell puts it under the census
that the `#[ignore]` was standing in for. Keep its book verbatim (`. ./common.sh` assigning
`sm.common/v1`, then `. ./alpha.sh` guarding on `sm.common/v2`) and its two assertions:
`LoadRoute::Reused` empty, `LoadRoute::Taken` naming `common.sh` twice.

**`fnd-sentinel-pin-horizon-is-stale`** (+SURE): the pin's own metadata says
`Horizon::Unscheduled { marker: "end-of-r31", why: "the ruling is queued on the human's
burndown" }`, but `30N:closed-guarded-source-projection-split` records the ruling CLOSED
2026-08-21 and `30O:dec-merge-guarded-source-with-closure-pins` schedules the work here. Greening
early is legal — the pin goes XPASS-to-promote and EXECUTE-B removes it — so nothing blocks. The
stale `why` is a conductor-tier register fix, not a builder edit.

### item-7 — `mech-acquire-and-ship-plain-sh` (EXECUTE-B, LAST)

The hard part is not acquisition; it is acquiring bytes WITHOUT acquiring meaning. Today one
vector (`paths`/`srcs`) serves both the snapshot and the definition universe, so simply pushing
the plain-sh file in would deliver tier-1 inclusion by the back door — `definition_table`'s
non-contract branch (`world.rs:788-797`) registers a FLAT `LoadProgram` of the file's funcdefs,
and `table.add` would put its names into `defs.names()`, turning `NoOpinion` into `Withheld` at
every consuming seat. That is the forfeit being cancelled by accident.

The type change that makes it unrepresentable:

- **`SourceRole` gains a member** (`cli/src/snapshot.rs:42-63`, which already DEMANDS the
  classification rather than defaulting it — `only-invocation-roots-are-ambient`). Name argued
  below: `SourceRole::PlainInclusion`.
- **`DefinitionTable` files a path as one of two things**, replacing the bare
  `by_path: BTreeMap<String, LoadProgram>`:

  ```rust
  enum Loadable {
      /// A dorc-lang file whose whole top level is a closed load PROGRAM.
      Program(LoadProgram),
      /// An ordinary sh file the controller READ and does not model. It binds nothing, its
      /// site havocs, and its bytes exist only to be mirrored.
      Included,
  }
  ```

  `program_at_key` answers `Some` only for `Program`, so `command_transfer`'s havoc arm needs no
  change at all — the wall is automatic. A new `included_at_key` is what `settled_account`
  (`funcenv.rs:1817-1874`) reads to `account.record` a `LoadSourcer::Book` occurrence WITHOUT
  running any program.
- **Acquisition** (`main.rs:478-483`) stops `continue`ing on a non-contract target: it records the
  bytes under the new role. `HelperIndex::build`, `build_dialect`, the lift lanes, and
  `definition_table`'s `table.add` loop all take the vectors that EXCLUDE `PlainInclusion` — audit
  every consumer of `source_srcs`/`oracle_srcs` and make the selection explicit at each. This is
  the single riskiest part of the item; a missed consumer is a custody widening.
- **Placement** (`cli/src/artifact.rs`) — the file is mirrored at its OWN authored relative path
  with NO import edit (the authored `.` already names it correctly), never bundled and never
  renamed, in every form that keeps files beside the plan. `bundle_files` (`:462-541`) branches on
  the occurrence's role; `bundle::project` (`bundle.rs:398`) must NOT strip a `PlainInclusion`
  source (`fnd-bundle-projection-strips-every-source`) — book-class bytes are mirrored verbatim.
- **Single-stream REFUSES**, naming the form: `inline_imports` (`artifact.rs:594-616`) answers
  `None` for a plain-sh load, so `auto` falls back to multipart and an explicit single-stream
  request refuses pre-network with `EXIT_ARTIFACT_UNSERVABLE`. That is
  `30P:principle-book-code-source-is-inclusion` tier 3 staying forfeited, and
  `KNOBS:kBACKFLIPS`'s verbatim-or-refuse weld.

**Acceptance OBSERVES the capability.** `load30-plain-sh-inclusion-ships` (committed red in this
MAP half) publishes an artifact set whose exec gate runs `plan.sh` from the generation ALONE; its
target run set is `ran: wombat note done`, emitted from inside the included file's own function,
which can only happen if `helpers.sh` was mirrored. Measured today: the case fails with exactly
`unresolved_generated_imports` naming `./helpers.sh`. The interim unit test
`a_plain_sh_source_walls_today_interim` (`main.rs`) is its twin and MUST be re-pointed when the
pin greens — it asserts `found.is_empty()`, which is precisely what stops being true.

### names-argued

- **"havoc" is a VALUE, `Top` is a LATTICE ELEMENT.** They are not synonyms and the code must not
  let them become synonyms: `EnvStack::Top` keeps meaning "the stack itself is unknown, shape
  included" (an unparsed node, a depth mismatch), while `havoc_names` means "every known name is
  ⊤ in this frame". Conflating them is exactly the bug being fixed. Two words, kept apart.
- **"spelling"** — a two-to-three-word name (`ScriptSpellings`, `Spelling::Slashless`) because
  the bare word is overloaded: this corpus uses "spelling" for how an AUTHOR writes a Dorc idiom
  in sh (`mark spelling`, `emit-never spellings`). Here it means how an INVOCATION names the
  script. `ScriptSpellings` is the disambiguator; do not shorten it to `Spellings`.
- **"exact"** — kept, because `30P` and `30I` both already use it as a term of art
  (`rul-load-head-is-exact-or-havoc`, `dependency-guarded-source-exact`, `exact target closure`)
  and inventing a synonym would fragment the vocabulary. It is a CROSS-DOMAIN GLOSS risk
  (linkers say "exact" of relocations) and the containment is that it never appears without its
  charter slug nearby.
- **`ParamExpansion` over `ParamComplex`** — "complex" says only "not simple", which is what made
  it acceptable to throw the body away. The new name says what it carries.
- **`SourceRole::PlainInclusion`** over `PlainSh`/`Included`/`BookSourced` — `PlainSh` names the
  file's dialect and says nothing about what Dorc does with it; `Included` collides with `30P`'s
  `principle-book-code-source-is-inclusion` in a way that would read as "spliced";
  `BookSourced` is already taken and means something else. `PlainInclusion` names the ACT and the
  tier at once. Two words, arguable, conductor may overrule.
- **NOT used, deliberately**: `placement` (reserved for semantic arrangement), `layout`
  (reserved for textual emission), `lift` (the static lift of oracle text), `Grade` (claim tier),
  `provenance` (the derivation DAG) — `30P:rul-emission-is-the-umbrella-name` and the brief's
  naming rider.

### types-make-unrepresentable — product-wide

- **`ScriptSpellings` on `DefinitionTable`** makes unrepresentable, PRODUCT-WIDE: a load answer
  computed against a `$0` that disagrees with the cwd the same table resolves against (they are
  one value); a `$0` read from a host or a shell (nothing constructs one from a payload); and a
  resolution that consulted only one spelling where two are live (the predicate takes the set,
  never a member). Still ADMITS: a book invoked through a symlink or found on `PATH`, where sh's
  real `$0` is neither spelling — disclosed, and the reason `30P` says "never realpath'd".
- **`Loadable::{Program, Included}`** makes unrepresentable: a source whose bytes the artifact
  ships but whose definitions also bind (the two live in different variants); and a plain-sh
  target silently acquiring a flat declaration list (the non-contract branch no longer produces
  a `LoadProgram` at all). Still ADMITS: a dorc-lang file that FAILS its load-inert lint, which
  today is refused at acquisition rather than filed as `Included` — a deliberate hole, since
  filing it would make lint failure a route to shipping.
- **`OperandAnswer` + `HavocCause`** make unrepresentable: a havoc with no attributable cause
  (the variant carries one), and a "resolved" answer that never named a snapshot key. Still
  ADMITS: a `Dead` verdict that is wrong because the target's tree is not the generation Dorc
  built — the mirroring contract is what makes it sound, and it is stated, not typed.
- **`ParamOp`** makes unrepresentable: a trim whose pattern was discarded (the pattern is a field),
  and an operator silently treated as identity (`Unmodelled` is a variant a consumer must match).

### acceptance — every test by name, with its CFG shape

RED CELLS COMMITTED IN THIS MAP HALF (`3b2e72d4`, `33406bb8`), each verified red for the right
reason and each greening in the item that owns it:

| cell | seat | CFG shape exercised | greens with |
|---|---|---|---|
| `p-x-unknown-source-havocs-the-cwd` / `a_relative_source_below_an_unknown_one_cannot_be_identified` | `cli/src/main.rs` | two straight-line top-level `.`s: a ⊤ operand then a cwd-relative literal | item 1 D2 |
| `p-x-unknown-source-havocs-shell-options` / `an_unknown_source_leaves_errexit_unknown_below_it` | `cli/tests/sh_parity.rs` | three straight-line top-level commands; the fallible one is NOT last, so an edge to `Exit` can only be the errexit failure-edge | item 1 D3 |
| `p-x-dollar-zero-slashless-book-path-resolves` / `a_slashless_book_path_still_names_the_books_own_directory` | `cli/src/main.rs` | one top-level `.` whose operand is one double-quoted word (expansion + literal tail), helper call below | item 2 |
| `p-x-computed-dot-parses-and-havocs` / `a_computed_dot_operand_parses_and_havocs_instead_of_refusing_the_book` | `cli/src/main.rs` | one top-level `.` whose operand word carries a `CommandSubst` part | item 3 |
| `p-x-plain-sh-inclusion-ships-beside-the-plan` / `a_plain_sh_source_is_acquired_and_placed_without_being_analyzed` | `cli/src/main.rs` | one top-level `.` of a literal relative operand, helper call below | item 7 |
| `load30-plain-sh-inclusion-ships` (e2e, ARTIFACT_SET + mocks) | `cli/tests/` | published-generation exec: the run set can only be non-empty if the mirror happened | item 7 |
| `p-x-glob-load-members-are-order-unknown` / `glob_members_meet_over_every_order_of_their_load_programs` | `cli/src/main.rs` | `for` over a glob word, body is a single `.` of the iteration variable | the deferred glob lane (stays red) |

GREEN DOMAIN PINS added beside them (a cell that would XPASS is a green test, never a pin):
`a_positional_parameter_below_an_unknown_source_stays_unknown` and
`a_site_below_an_unknown_source_is_still_modelled_as_reached` (both `cli/tests/sh_parity.rs`).

GREEN AND MUST NOT MOVE: `a_name_bound_only_before_an_unknown_source_is_withheld_after_it` ·
`an_unknown_source_keeps_defensive_emission_and_its_wall` ·
`a_book_subshell_helper_suspends_like_an_ambient_one` ·
`the_environment_names_a_definition_per_frame_within_one_file` ·
`a_host_conditional_oracle_definition_licenses_nothing`. RE-POINTED, not deleted, when their
targets green: `a_plain_sh_source_walls_today_interim` (item 7) and
`a_book_subshell_helper_suspends_like_an_ambient_one` (item 5).

EXECUTE-A additionally owes, once the codes exist: a cell asserting
`script-relative-load-dies-slashless` fires on a bare `${0%/*}` book, and one asserting
`slashless-source-searches-path` fires on `. helpers.sh`. Both are e2e `expect-diagnostic:`
needles (the needle validator refuses a slug the catalog does not hold, which is why they could
not be committed red here).

### commit-order

**EXECUTE-A** (load heads). Each step is independently green.

1. `ParamOp` decode in `syntax` (lexer + AST), with `empty_defaulted`'s consumer converted to the
   projection. No behaviour change yet — the analyzer still ⊤s every expansion.
2. `ScriptSpellings` on `DefinitionTable`, minted at `world::definition_table`. Still unread.
3. The operand evaluator + `OperandAnswer`/`HavocCause`, wired into `load_sites` and
   `command_transfer` behind `literal_text`. Greens
   `p-x-load-operand-param-expansion-of-dollar-zero` and
   `p-x-dollar-zero-slashless-book-path-resolves`.
4. The two lint `DiagCode`s (`message: None`) + their e2e needles.
5. D1 pointwise havoc. Greens `p-x-unknown-source-is-a-point-havoc`. **CHECKPOINT-WORTHY**: this
   is the licensure-shifting commit; report the two anti-regression greens explicitly.
6. D2 cwd clobbers. Greens `p-x-unknown-source-havocs-the-cwd`.
7. D3 errexit ⊤ at a `.`. Greens `p-x-unknown-source-havocs-shell-options`.
8. The parser change + the post-analysis fail-fast + `EXIT_LOAD_UNRESOLVABLE`. Greens
   `p-x-computed-dot-parses-and-havocs`. Do this LAST of A: it makes computed operands reachable
   for the first time, so it exercises everything above.

Order rationale: 5 before 6 would open the cwd hole for one commit; 6 before 5 is inert. Both are
before 8 because 8 creates new havoc sites.

**EXECUTE-B** (custody and inclusion).

1. `30Pc:required-repair` — `assigns.is_empty()` on `absorbable`, all five of its steps including
   the deliberate-removal falsification. Small, local, and it is a live correctness defect.
2. `p-x-helper-unset-f-across-files` — `HelperIndex::resolve` consults the frame.
3. `p-x-regional-helper` — `closure_for` takes a site; the book census becomes region-aware.
4. `p-x-definition-grade-keying` — per-definition lift arity in `oracle/src/{predict,verdict}.rs`.
5. Guarded-source fidelity: carry `literal` through `decide_guard`/`sentinel_decides`; re-author
   `176e0818` as a pin cell; promote `p-x-sentinel-value-conjunct`.
6. `mech-acquire-and-ship-plain-sh`, LAST, because it touches `cli/src/artifact.rs`.

### shared-file-ledger

`cli/src/main.rs` is the file `30O`'s schedule names as shared. It is not the only one.

| file | this lane's ranges | the other lane | assessment |
|---|---|---|---|
| `analysis/src/value.rs` | **NONE** | loop lane: `member_argv` (field `:95`, accessor `:180-183`, `members_pass` `:863`), `inline_pass` `:1086` | The `$0` model deliberately does NOT enter `ValueFlow` (a single-valued plane cannot carry two spellings). The scout's guess that this lane would touch `variable_before` (`:234`) is SUPERSEDED — the evaluator READS `SourceLiteralPlane`, which reads `variable_before`, and changes nothing in `value.rs`. Conflict surface: zero. |
| `cli/src/main.rs` | `:1042-1049` (outcome ranking) · `:110-150` (exit consts) · `:478-483` (acquisition) · the `acquisition_tests` module `:4101+` | planner lane: `why`/`world` call sites; loop lane: none known | Small, disjoint hunks. |
| `cli/src/artifact.rs` | EXECUTE-B only: `book_loads` `:339-382` (the `30Pc` repair) · `bundle_files` `:462-541` · `mirrored_files` `:543-592` · `inline_imports` `:594-616` | planner lane: the same file, for hoist/munge | REAL overlap. Mitigated by fold order (`loop → planner → load`) and by EXECUTE-B doing item 7 LAST. `30O`'s own item-3 text already says "coordinate with the planner lane". |
| `analysis/src/funcenv.rs` | `:63-146` · `:480-560` · `:1600-1810` · `:1926-2160` | none | Ours. |
| `analysis/src/cfg.rs` | `errexit_after` `:1826` + `errexit_toggle` `:1842` | none | NOT in `30O`'s stated touch list for this lane; added by D3. Two-line arm. |
| `syntax/src/{lexer,ast,parser}.rs` | `lexer.rs:640-716` · `ast.rs:230-245` · `parser.rs:1075-1093` | none | NOT in `30O`'s stated touch list; the scout flagged this and it is confirmed — item 2 cannot be done in `analysis` alone. |
| `oracle/src/closure.rs` | EXECUTE-B: `:266-295` · `:297-340` · `:534-583` | none | Ours. |
| `cli/src/snapshot.rs`, `cli/src/bundle.rs`, `cli/src/world.rs` | EXECUTE-B item 7 | planner lane touches `world.rs` | Small. |

### the-riders — `30O`'s list, discharged

- `p-x-blessed-toplevel-conditional` is NOT included (it waits on
  `28Q:res-dot-blessing-is-engine-side`). Untouched, still red, horizon untouched.
- `dirname`-headed operands beyond point havoc are NOT included. The static-predict tier
  (`30P:rul-static-predict-sites-loads`) needs an authored `dirname__predict` and there is no
  stdlib to carry one. This lane makes the operand PARSE and HAVOC with a hint; nothing more.
- The slashless-OPERAND lint (`. helpers.sh` is a PATH search) IS included, as
  `slashless-source-searches-path`, a `DiagCode` with empty prose.
- STRUCK riders, per the conductor's correction and per `30P` outranking `30O`: the three-state
  POSSIBLE / EXACT / ENGINE-SELECTED identity (the model is EXACT-or-havoc, nothing between);
  "ship + wall" for `dirname` heads; and any reading of
  `rul-guard-resolves-like-its-mutation` as [PROPOSED] — it is TYPED law.

### deviations — OPEN for the conductor

- **`dev-lint-cells-wait-for-their-codes`** — the brief asks for the off-ramp lint as a committed
  red cell. It is not committed. A red cell must compile and be red for the RIGHT reason today;
  an e2e `expect-diagnostic:` needle naming a slug the generated catalog does not hold is REFUSED
  by the validator (a dead slug is an error, not a failing assertion), and no in-process
  observable exists for a diagnostic that has no emitter. The lint's obligation is stated as
  EXECUTE-A acceptance instead, with both loom skeletons written out above. Reasoning offered;
  conductor re-derives.
- **`dev-slashless-cell-is-the-controllers-path`** — the brief's "a slashless-spelling test" is
  committed as *the controller's own book path may be slashless*, not as *the two spellings
  disagree*. The disagreement cell would XPASS today (nothing evaluates the operand at all, so
  "nothing resolves" is already true), which is a loud failure, not a pin. The committed cell is
  red today and is the one a slash-bearing-only implementation would get WRONG.
- **`dev-case-form-is-not-in-r30`** — the brief's acceptance lists
  `case $0 in */*) … *) here=. ;; esac` resolving EXACT. It is not scheduled here. That form
  moves the computation out of the word and into CONTROL FLOW, so it needs a per-spelling fold:
  a new member of `dec-decidable-set-v0` (a `case`-pattern test over a controller-known
  scrutinee) AND a second, per-spelling solve whose results meet. Both are license-review-tier
  and the second is a structural change to `funcenv::analyze`'s round model — out of proportion
  to a lane whose other seven items are additive. The word evaluator this lane builds is the
  prerequisite either way. Flagged rather than silently dropped.
- **`dev-domain-cells-that-would-xpass-are-green-tests`** — the brief asks for one cell per havoc
  domain "even if trivially ⊤ today". Positionals and termination are already answered correctly,
  so a pin over them would XPASS loudly. They are committed as green anti-regression tests whose
  doc comments name the domain and say why they are not pins.
- **`dev-cfg-and-syntax-join-the-touch-list`** — `analysis/src/cfg.rs` (D3) and
  `syntax/src/{lexer,ast,parser}.rs` (items 2 and 3) are not in `30O`'s stated touch list for
  this lane. Both are unavoidable; the parser half is explicitly authorized by the human's
  2026-08-22 lean, the `cfg.rs` half is two lines.

### tc-flags — cross-cutting, for the human

1. **`tc-dollar-zero-spelling-asymmetry`** (§item-2) — is the slashless spelling a RESOLVING
   spelling (both must resolve for EXACT) or a LINT spelling (Dorc invokes the slash-bearing one;
   the other only lints and only denies EXACT when it resolves a DIFFERENT file)? The strict
   reading makes `load30-point-havoc-and-script-relative` unresolvable under the cwd domain and
   so unable to green in r30. Lean: LINT spelling. Strawman in §item-2.
2. **`tc-computed-dot-complaint-shape`** (§item-3) — does a computed `.` fail-fast at all, and
   per-site or whole-run? Lean: keep today's outcome, move only the tier. Strawman console
   transcript in §item-3.
3. **`tc-cwd-havoc-costs-relative-acquisition`** (§item-1 D2) — the cwd domain means a book that
   sources anything unknown at the top and then sources its own oracles RELATIVELY loses those
   oracles entirely (not just their licenses: they are never acquired, so never mirrored, so the
   apply dies at the `.`). Sound, and the safe direction, and possibly surprising enough to want
   an explicit hint. Strawman:
   ```sh
   . /etc/os-release          # unknown: cwd is now unknowable
   . ./oracles/docker.dorc.sh # ← no longer acquired at all
   docker run …
   ```
   Lean: build it, and give the second `.` a hint naming the first line as the cause.

### proposed-steering-and-register-edits (conductor applies; builders edit no `CLAUDE.md`)

- `syntax/CLAUDE.md syntactic-top-triggers` — the clause "command-substitution and arithmetic
  targets stay parse-⊤" becomes false at EXECUTE-A step 8. Proposed replacement: *"a `.` operand
  carrying a command substitution or arithmetic expansion PARSES (a rich AST) and routes to the
  analyzer's load plane, which answers ⊤ ⇒ point havoc; the pre-network complaint is the cli's
  (`30P:rul-floor-valid-text-never-parse-fails`)."*
- `analysis/CLAUDE.md` — a new bullet under "Law — the dangers", proposed slug
  `rul-havoc-is-pointwise-never-the-stack`: *"an unresolvable `.` raises every KNOWN name to ⊤ in
  the innermost frame and nothing else; `EnvStack::Top` stays reserved for a stack whose SHAPE is
  unknown (an unparsed node, a depth-mismatched join). A later unconditional definition re-binds
  by sh's last-wins, and a name outside the unit's universe still answers `NoOpinion` —
  `rul-guard-resolves-like-its-mutation` puts an unknown file redefining a tool on the admin's
  side of the horizon."*
- `analysis/CLAUDE.md funcenv-reads-source-literal-plane-only` — a rider: the load-operand
  evaluator reads the AST for STRUCTURE and routes every variable read through the plane; `$0` is
  a controller-held constant on `DefinitionTable`, not a variable.
- `cli/CLAUDE.md` — a rider under `bundle-projection-is-pre-contact-and-not-placement`: a
  `PlainInclusion` source is mirrored BYTE-VERBATIM and never stripped, never bundled, never
  renamed, and never inlined into a single stream.
- `FORFEITS.md forfeit-plain-sh-inclusion-analysis` — its REDS row gains
  `p-x-plain-sh-inclusion-ships-beside-the-plan` and `load30-plain-sh-inclusion-ships`. (Strictly
  these are reds for the CAPTURED half, so the conductor may prefer to note them as the row's
  landing evidence rather than as forfeit-reds.)
- `FORFEITS.md forfeit-book-dynamic-load-analysis` — its REDS row gains
  `p-x-computed-dot-parses-and-havocs` and `p-x-dollar-zero-slashless-book-path-resolves`.
- `30O:register-and-steering-debt` — add `fnd-sentinel-pin-horizon-is-stale` (the
  `p-x-sentinel-value-conjunct` `why` text contradicts `30N` §4).

### context-other-lanes-must-maintain

- The loop lane: this lane touches `analysis/src/value.rs` NOT AT ALL. `30O`'s schedule header
  ("`cli/src/main.rs` is the one shared file") is right for this pair after all, contra the
  scout's flag.
- The planner lane: `cli/src/artifact.rs` IS shared, in EXECUTE-B's last commit only. The
  `30Pc` repair (`assigns.is_empty()` on `absorbable`) lands there first and is small; if the
  planner lane wants it earlier, it is a clean two-line lift.
- Everyone: `load31-punted-load-shapes`'s `expected.ran` is NOT being compared today
  (`fnd-plain-sh-dies-on-the-host`). Do not read it as an assertion until the case's structural
  gates pass.
