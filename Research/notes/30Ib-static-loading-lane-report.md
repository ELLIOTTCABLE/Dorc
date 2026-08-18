# 30Ib — the static-loading lane report (LIVING)

> Builder's as-built report for the `Research/plans/30I` lane. `30I` is the semantic
> authority and the work order; this file is *as-built state* — where the load model
> lives, what consumes it, what is pinned where, what is OPEN, and what a fresh
> builder does next. Not a re-plan and not a chronology (git carries that).
>
> A successor who has read `30I` and nothing else should resume from this file
> without re-deriving any decision below.

## §0 — Where the lane stands

Work-order items **2, 3 and 4 are landed**, with the single exception named in
§4 `dev-cross-custody-refusal-not-built`. Items 5, 6 and 7 are **untouched** — no
bundle projection, no artifact forms, no XFAIL promotion, no golden work. The
three `load30-*` specimens remain XFAIL with empty `head-expected.ran`, and
`floor30-dot-loader-function-errexit` is byte-identical.

## §1 — The crossing, as found (`30I` work-order §1)

Measured in-tree at `7d04f066`, before any edit.

- **Filesystem reads / source-vector appends** — all in `cli/src/main.rs`:
  `resolve_oracle_paths` · the `oracle_srcs` read · `read_sourced_oracles` · `read_books`.
- **`funcenv` load inputs** — `load_sites` read a `.` operand via
  `SourceLiteralPlane::literal_text(node, 1)` and matched the raw text against
  `DefinitionTable::by_path`, keyed by the CLI-spelled path. No cwd anywhere; no
  model of a sourced file's own top level.
- **Include-tree / custody** — `cli::sourcing::include_tree` plus
  `core::CustodyClosures::from_edges`, built identically in both drivers.
- **Emission seats carrying loaded bytes** — `oracle::closure::HelperIndex` ·
  `plan::build_vouches` · `compile_probe`'s three ship closures ·
  `Plan::pinned_definitions` → `Plan::render_apply` (`SpanEdit` over book bytes).
- **Spine** — `SpineLoadDecision` records a name plus custody, no load occurrence.
- **Provenance** — `core::prov` is the collapse arena; `DefinitionId` is the row key.
  Neither carried a multi-stage locator.
- **Plan projection** — `Plan::render_apply` is the only artifact assembler;
  `main.rs` prints probe-then-apply to stdout and writes no file.

## §2 — As-built: the load model's seats

Each row is the ONE home for its question. A second implementation of any of these
is the thing `30I:impl-one-builder-one-lane` exists to prevent.

### `core::loadpath` — where a `.` operand lands
`Cwd` is the CONTROLLER-side modeled working directory (named for its side: the
target-side execution cwd is a different question and needs its own type).
`Cwd::resolve_dot` is sh's `.` rule — slash-less ⇒ `None` (a `PATH` search, outside
v0); `Cwd::resolve_operand` is the plain path-operand join and is the CANONICAL KEY
every loaded source is filed under. `Cwd::unknown()` is the edge's "could not say",
under which relative operands resolve nowhere. Pure text: no filesystem, no
environment, so both kernels and the edge share one rule.

### `cli::snapshot::StaticLoadSnapshot` — the one immutable authored input
Ordered `(path, bytes)` vector plus a per-source `SourceRole`
(`Book` / `NamedLoad` / `BookSourced`). ROLE IS CARRIED, never derived from
position — reading "book == last" in a consumer is what would fossilize one book at
the end. `is_ambient(file)` answers from the role. `key_of` / `source_at_dot_target`
are the canonical-identity seats. Built purely; the reading is at the edge.

`snapshot::book_load_targets` / `snapshot::book_reached` are the pure "which loaded
sources does a book `.` reach" walk, shared by the binary and the loom's in-process
driver so a case and a run partition identically.

### `analysis::load` — a loadable file's top level as a closed program
`LoadProgram` = `Vec<LoadStep>`; `LoadStep` = `Define | Assign | Control`;
`LoadControl` = `UnsetFunctions | Load{target,span} | Guard{function,negated,then_,else_}`.
The split is load-bearing: **`LoadControl` has no declaring variant**, so a
declaration inside a guard branch is unrepresentable, not merely refused. That
class is a measured wrong-elision route (the dialect lift sees role headers only as
top-level items), pinned by `sh_parity.rs`'s
`a_host_conditional_oracle_definition_licenses_nothing` and its expected-fail twin.
`LoadTarget` keeps the operand UNEXPANDED — the root lives in the caller.

### `oracle::load_inert` — the admission gate
Widened to admit the canonical include guard (`command -v <literal name>`, optional
`!`, optional redirects, branches of load control only) and top-level `unset -f`.
`elif` is refused (nest instead). `include_guard` / `unset_functions` /
`item_is_static_load` are the syntactic readers `cli::world::load_control` turns
into steps — one reading, two consumers.

### `analysis::funcenv` — the interpreter
`DefinitionTable` carries the `Cwd` and maps CANONICAL path → `LoadProgram`.
`run_program`/`run_control` interpret a program in place at each load site.
Guard decision is ASYMMETRIC and that is the safety argument:

- frame names a LIVE definition ⇒ TRUE unconditionally (a live function shadows
  every builtin and binary; no host can make the query fail);
- frame proves UNDEFINED ⇒ FALSE only for a ROLE-shaped name
  (`28M:dec-decidable-set-v0`'s warrant: nobody ships a binary called
  `apt_get__is_converged`);
- anything else ⇒ both branches JOIN (can't-say, withholds).

`LOAD_DEPTH_CAP` plus a visiting-stack cycle guard; a diamond is followed twice, a
cycle answers ⊤. `FuncEnv::wanted_loads()` is the loader's own account of paths it
named but does not hold — a settled post-pass (`wanted_after`), never an
accumulation inside the transfer. `FuncEnv::sourced_paths()` exposes the load acts.

### `cli` edge — acquisition driven BY the loader
`main.rs::read_book_sourced` loops: build a snapshot, build the definition table,
run `funcenv::analyze`, read whatever `wanted_loads()` names that satisfies the
dorc-lang contract, repeat until nothing new (`ACQUISITION_ROUNDS_CAP`). There is
no second resolver at the edge to drift from the engine. Only MARKED
contract-satisfying targets are admitted; everything the loop appends is
`BookSourced`.

### `analysis::value` — the variable window
`ValueFlow::variable_before(node, name)` retains the solve's per-node environments;
`SourceLiteralPlane::variable_text` is the load plane's window onto it. Carries the
grade obligation `funcenv-reads-source-literal-plane-only` will need when
`seam-re-bind` lands (see §5).

### `aid::locator` — the multi-stage locator DAG
`Locator` is an arbitrary DAG of `Stage` = `Authored | Loaded | Copied | Generated
| Claimed`, with fan-in and `chain_from` deduplication; a later stage COMPOSES an
edge rather than overwriting history. `BundleOriginClaim` is sealed — private
field, one text constructor, one `&str` accessor, no conversion to any identity —
and a lexical fence test asserts the module names no authority type at all.

### `cli::provenance` — locators from real run data
`LoadActs::of(snapshot, cfg, book, env)` reads the load acts off the settled
environment; `locator_for` builds `Authored ← Loaded`; `render_chain` renders
`path:line` outermost-first. **Test-only consumer today** — see §5.

## §3 — Scope lines held (read before widening anything)

- **No un-walling.** A book's `.` and a book-position `command -v` stay unmodeled
  ⇒ ⊤ ⇒ they wall. The loader may resolve WHICH file a `.` names while the
  analyzer still walls the emitted command; neither answer contaminates the other.
  Un-walling is a licensure widening routed to the human
  (`FORFEITS:forfeit-command-v-poison-wall` CAPTURE; `30G:b8-book-side-unwalling`),
  and the specimens do not need it — their `expected.ran` assert RUN SETS, and a
  walled line still runs.
- **Only MARKED dorc-lang targets enter the load model.** A book `.` of an
  unmarked file is never opened, never modelled, never rewritten. This is what
  keeps `floor30-dot-loader-function-errexit` byte-identical: its `child.sh` is
  unmarked and carries a top-level `return` plus a failing command.
- **Guard branches may not declare** — gate AND type, deliberately both.
- **No durable growth.** The ambient/book-sourced partition is RE-DERIVED every
  run, replays included, rather than carried (`rul-durable-contents-reviewed-before-design`).

## §4 — Deviation ledger (OPEN; for conductor/human adjudication)

None is self-endorsed. Each names what was done and what the alternative was.

1. **`dev-analysis-cwd-pinned-to-the-case-dir`** — the e2e harness now runs `dorc`
   with `current_dir` = the case's (materialized) directory; `Harness::dorc` takes
   the directory as a parameter so every call site names it. Reason: cargo sets a
   test process's cwd to the PACKAGE root, under which no case's `./helpers.sh`
   resolves, and a loom case materialized into a scratch dir would resolve
   differently again. The case directory is also the shape an admin gets by
   running `dorc` where their files are, and it makes a case relocatable.
   Consequence: the three `load30-*` books were re-spelled
   `SM_ORACLE_ROOT=crates/cli/tests/<case>` → `SM_ORACLE_ROOT=.`. No target run
   set changed, `head-expected.ran` stayed empty, and the whole corpus is green
   with zero golden drift. The alternative (cwd = `spike/`) keeps the specimen
   bytes but ties every case's analysis to the repo layout and cannot work for a
   materialized loom case.

2. **`dev-slash-less-dot-is-now-unresolvable`** — `. foo.oracle.sh` (no separator)
   resolves NOWHERE, per `30I:rul-dot-resolves-as-sh`. Previously the target text
   was matched against the loaded-path table directly, so it resolved.
   `pin28-variable-resolved-source-loads` spells exactly that and now pins an
   unresolvable load while its header prose describes a resolving one. Its goldens
   did not move (the site walled either way), so the corpus is green and the drift
   is INVISIBLE. **Owed**: re-spell that book to `. "./$PKG.oracle.sh"` and
   re-bless, which is golden work and therefore not this builder's. The property
   itself is pinned natively both ways in `funcenv`
   (`a_relative_target_resolves_against_the_modeled_working_directory`,
   `a_slash_less_target_is_a_path_search_and_resolves_nowhere`).

3. **`dev-guard-false-direction-fenced-to-role-names`** — the include guard decides
   TRUE from a live definition unconditionally, but decides FALSE only for a
   role-shaped name. `30I:rul-include-guards-are-load-semantics` is TYPED and reads
   as wanting the FALSE direction for ordinary helper names too (its own §2.2
   example guards `example_common_query`). Deciding it would model a fallback as
   loaded on a host where a binary of that name answered the query and the other
   branch ran — the mis-attributed class. Widening the fence is a licensure
   question with an owner above this component (`inv-superposition`), so it is
   flagged, not settled. Consequence: a guarded ordinary-helper name binds ⊤ rather
   than to the fallback. Nothing licenses off ⊤, so the direction is withholding;
   the specimens' run sets are unaffected because they depend on the guard at
   RUNTIME, not on the engine deciding it.

4. **`dev-cross-custody-refusal-not-built`** — work-order item 3's last clause (the
   v0 unannounced cross-custody cell: mint the pre-network closed outcome and its
   structured diagnostic data) is NOT built. Two reasons, both structural rather
   than difficulty: (a) making it a refusal changes
   `emit30-cross-custody-plural-helper-suspends` from "suspends, apply is the book"
   to "no mutation-authorizing plan", which is golden movement, and golden work was
   removed from this builder's remit; (b) `30I` §3.4's case 2 (an authored guard as
   an attributable acceptance chain) is a licensure WIDENING against the
   human-ruled `rul-vouch-reaches-own-custody-only`. Building only the classifier
   without a consumer would be the multi-phasic scaffolding the project's own law
   warns against. Design, ready to implement: classify a voucher's cross-custody
   reach as `deliberate-external-utility` (the call goes through `command`),
   `explicitly-selected` (the voucher's own file carries an include guard naming
   it, which the load program now makes visible), `explicitly-sourced` (the
   existing custody closure), or `merely-live` (none of those ⇒ the refusal).

5. **`dev-ambient-include-guards-are-not-evaluated`** — a package named with `-o`
   loads through the AMBIENT prefix, which applies a flat declaration list rather
   than running its `LoadProgram`. So an ambient package's include guard is not
   evaluated, and its guarded dependency is bound unconditionally (the dependency
   file is itself appended as an ambient oracle by `read_sourced_oracles`). Before
   this lane the gate refused guards outright, so the shape did not exist; admitting
   guards created it. The over-approximation can shift a winner only when an
   EARLIER ambient file defines the guarded name, where the cross-unit shadow
   refusal already withholds a role family (a helper collision is reported rather
   than withheld). The fix is to run ambient programs at the `Entry` transfer,
   which needs a decision about what an unresolvable ambient load does — havoc the
   entry state (consistent with `rul-unloadable-is-unlicensed`) or contribute
   nothing and let the existing composition suspension carry safety. That is a
   licensure-relevant call and is left open.

6. **`dev-one-commit-lumped-two-changes`** — `827bc59f` carries both the
   `LoadControl` type split and the locator DAG + `cli::provenance`. They should
   have been two commits; the rest of the lane is one coherent slice per commit.

## §5 — Open questions and owed work

1. **The successor's FIRST task** (`30I` work-order §4's force-now proof, explicitly
   deferred by the conductor): carry one locator chain into a REAL diagnostic
   render. `cli::provenance` is consumed only by its own tests today; the
   interesting chain runs through the bundle, which is item 5.
2. **Variable-rooted dependencies do not mint CUSTODY.** `include_tree` resolves
   LITERAL targets only, so a package whose dependency is spelled
   `. "$ROOT/dep.sh"` is marked `unresolved` and its vouches SUSPEND — measured
   live against the real binary (`vouched-composition-not-present` at the
   entrypoint). The loader resolves such a target correctly; what is missing is
   reporting the edges it walked so `include_tree` can consume them instead of
   re-resolving. Shape: extend `wanted_after`'s post-pass to collect
   `(sourcer key, target key)` pairs and feed them to `include_tree`. The blocker
   to doing it naively: the AMBIENT prefix's programs are never run (see §4 row 5),
   so an ambient package's literal edges would vanish and
   `pin28-helper-package-entrypoints-lift` / `pin30-swapped-entrypoints-source-the-helpers`
   would lose their custody. Fix §4 row 5 first.
3. **A package's nested `.` is not yet a locator EDGE**, so a
   dependency-of-a-dependency resolves to two stages rather than three. Same
   mechanism as (2).
4. **`ValueFlow::variable_before` has no per-variable `ValueGrade`.** Today every
   non-⊤ value is program text (captures are ⊤ and the value plane runs before the
   probe), so the wall is vacuously held. When `core`'s `seam-re-bind` folds
   captured values back in, that accessor MUST gate on the grade or a host-spoken
   value can site a load. Disclosed at both seats.
5. **Several `--book` operands are still `\n`-concatenated into one text.** The
   whole analysis below assumes one `Ast`/`Cfg`/`ValueFlow` and `render_apply`'s
   span edits over one string. `SourceRole` stops the "book == last" assumption
   spreading, but undoing the concatenation is an arc, not a slice.
6. **A replay's ambient partition.** `book_reached` is re-derived from the book, so
   a replay partitions as its original did wherever the book's loads are literal or
   its roots are book-set. A source that is BOTH invocation-named and book-sourced
   classifies `BookSourced` (withholding). The sharper answer needs a boundary a
   replay can recover, which today it cannot without growing the durable.
7. **`p-x-helper-unset-f-across-files` is now one line from greening.** Admitting
   top-level `unset -f` forced a REAL model of removal into
   `oracle::closure::HelperIndex::record` (the `p-helper-unset-f` pin caught the
   widened allow-list immediately, exactly as its own doc predicted it would). That
   model is deliberately PER FILE, so the cross-file target still fails as expected.
   Dropping the `declaration.file != file` filter closes it — a later file's
   top-level `unset -f` removing an earlier file's declaration is what a shell does,
   and only unconditional top-level removals reach that seat. Greening it is a
   PROMOTION (its `PINS` row must go in the same change), which is item 7's, and it
   is on `30G` §4 item 9's re-check list.
8. **A floor cell for textual inlining is owed before item 5 flattens anything.**
   `fnd-loader-function-errexit-diverges` refutes generated loader FUNCTIONS; it
   says nothing about inlining a marked, load-inert child at its source position.
   That inlining looks sound (no top-level `return` is representable in an admitted
   marked file, and a funcdef cannot fail), but it is ARGUED, not MEASURED.
   Minting the cell is `mise run bless:floor`, which is orchestrator-only and
   WSL/*nix-only.

## §6 — What is pinned where

**Native (fast, at the ownership seat):**

- `core::loadpath` — the whole `.`-resolution rule, both operand kinds, the unknown
  cwd, Windows spellings, lexical normalization.
- `cli::snapshot` — book-last ordering, role/ambience, canonical dot-target
  resolution, `book_reached` (root value flow, unmarked targets excluded, regional
  loads found).
- `oracle::load_inert` — the canonical guard admitted (four spellings), branches may
  not declare/assign/run, only `command -v` opens a guard, `unset -f` in and bare
  `unset` out.
- `analysis::load` — operand expansion against the loading context, file-local
  constants shadowing the caller, declarations flat while loads are not.
- `analysis::funcenv` TABLE 5/6 — cwd-relative resolution across three cwds,
  slash-less unresolvable, root reaching a guarded dependency, unset root
  resolving nowhere, both guard directions, the diamond, one entrypoint at two
  frames, source cycle, `unset -f`, package constants.
- `aid::locator` — two-stage chain, composition, fan-in dedup, scaffolding without
  origin, the claim seal, the authority-type fence.
- `cli::provenance` — a definition in a loaded package resolving through its load
  act; an ambient oracle resolving to its own bytes alone.
- `cli::main::acquisition_tests` — the edge loop opening exactly the entrypoint and
  its guarded dependency (and not a co-resident stranger), an unmarked target never
  opened, an invocation-named oracle staying ambient.

**e2e:** unchanged. The three `load30-*` remain XFAIL; `floor30-*` byte-identical.

## §7 — Next steps into item 5

In this order, because each unblocks the next:

1. Fix §4 row 5 (run ambient programs at `Entry`), deciding the unresolvable-ambient-load
   question first — it is licensure-relevant and belongs to the conductor.
2. Then §5 row 2: collect resolved load edges in `wanted_after` and let
   `include_tree` consume them, closing custody for variable-rooted dependencies.
   Watch `pin28-helper-package-entrypoints-lift` and
   `pin30-swapped-entrypoints-source-the-helpers`.
3. Then item 5's bundle projection, keyed by static load occurrence, consuming the
   snapshot and the frame answers — never re-reading a path.
4. Compose bundle segments ONTO the existing locator edges (`Stage::Copied` with
   the authored stage as origin) and carry one chain into a real diagnostic —
   which discharges §5 row 1 and `30I` work-order §4 together.
5. Only then items 6 and 7.
