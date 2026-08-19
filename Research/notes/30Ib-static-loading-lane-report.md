# 30Ib — the static-loading lane report (LIVING)

> Builder's as-built report for the `Research/plans/30I` lane. `30I` is the semantic
> authority and the work order; this file is *as-built state* — where the load model
> lives, what consumes it, what is pinned where, what is OPEN, and what a fresh
> builder does next. Not a re-plan and not a chronology (git carries that).
>
> A successor who has read `30I` and nothing else should resume from this file
> without re-deriving any decision below.

## §0 — Where the lane stands

**Work-order steps 1, 2, 3 and 4 are landed.** Steps 5–8 are untouched: no bundle
projection, no locator-through-a-real-diagnostic, no artifact forms, no XFAIL
promotion, no e2e lowering. The three `load30-*` specimens remain XFAIL, now
spelled with the sentinel guard `30I` §2.2 carries; `floor30-dot-loader-function-errexit`
is byte-identical.

> The step numbering moved when `30I`'s work order was re-cut. This file's §1–§6
> were written against the OLD numbering (its "items 2/3/4" are the load model,
> the frame answers and the locator DAG); §7 was the runway into what are now
> steps 1 and 2. Read §8 for the current-numbering account.

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

## §8 — Builder 2: steps 1–3 as built

### Step 1 — a pre-source is a `.` (`30I:rul-pre-source-is-dot-prelude`)

`DefinitionTable::ambient` is now a vector of ROOTS (`AmbientRoot { key, defs }`),
not a flat `DefId` list, and the `Entry` transfer RUNS each root's `LoadProgram`
in invocation order through `run_ambient_prefix`. So an invocation-named package's
include guard decides, its own `.` reaches a dependency the invocation never
named, and its `unset -f` removes — the same interpreter a book `.` drives.

Three properties are load-bearing and each is pinned in `analysis::funcenv`'s
TABLE 6:

- **the flat fallback survives.** A root with no program on file — an unmarked
  source, which makes no dialect claim, or a path the modeled cwd could not
  canonicalize — binds its flat declarations exactly as the whole prefix used to
  (`a_pre_source_with_no_program_still_binds_its_declarations`). Nothing about the
  prefix's new richness may cost a plain file its binding.
- **one `locals` map spans the prefix**, because a shell's `.` leaves its
  assignments live for the next one; `visiting` is per-root, each being its own
  load act (`one_pre_source_sites_the_next`).
- **`contests` still walks the flat declarations** of every root, in the same
  order. That is deliberately unchanged: a guarded dependency the model does not
  bind still counts as an ambient shadow, which over-FIRES a diagnostic and
  withholds nothing.

This discharges the old §4 row 5 (`dev-ambient-include-guards-are-not-evaluated`)
and its open licensure question: nothing binds unconditionally through a
flat-declaration prefix any more, so there is no unresolvable-ambient-load call to
make. An unresolvable load inside a pre-sourced root havocs that root's
contribution (`rul-unloadable-is-unlicensed`), which is the withholding direction.

### Step 2 — the loader reports the edges it walked

`FuncEnv` gained `load_edges` (`(sourcer, target)` canonical-key pairs) and
`unresolved_sourcers`, collected by the settled post-pass (`settled_account`, née
`wanted_after`) alongside `wanted_loads`. `cli::sourcing::include_tree` now takes
`(&snapshot, &env)` and consumes them; its literal `.`-operand walk is GONE.

That closes custody for a variable-rooted dependency — `30I` §2.1's canonical
package shape, whose operand this seat holds no loading context to expand. Pinned
at `cli::sourcing`'s `a_dependency_sited_through_the_callers_root_takes_custody`.

Two things stayed on the cli side deliberately: the dorc-lang CONTRACT check at
BOTH ends of every edge (the loader binds an unmarked target's declarations, but
an unmarked target mints no edge and suspends its sourcer), and the book
exclusion. The loader itself records an edge only from a file whose own program
spelled the load, so a book `.` and an invocation-named root contribute none.

`main.rs` moved the `include_tree`/`HelperIndex` construction BELOW
`funcenv::analyze` — its first consumer was already below it, so the move is
positional only. `sourcing`'s tests now drive the whole pipeline (parse to cfg to
value to definition table to funcenv) rather than hand-building a tree; a
hand-built one would be the second resolver this change exists to delete.

**Not extended**: `settled_account` reports `wanted` for pre-sourced roots, but the
binary's acquisition loop for ambient dependencies is still the edge's literal
walk (`read_sourced_oracles` through `sourcing::top_level_load_targets`), which
runs BEFORE the book acquisition. A variable-rooted operand inside a pre-sourced
root resolves nowhere anyway (no book variable is live before line 1), so nothing
is lost today; a root that sites a dependency through a variable ANOTHER
pre-source set is the cell that would need the edge's loop to consume the loader's
account too.

### Step 3 — the CLI input surface (`30I` §2.4/§2.5)

Landed:

- **`--pre-source PATH`** (long-only, repeatable, CLI-ordered) replaces
  `-o`/`--oracle`, which are GONE from the analyze surface. `--oracle-dir` stays
  (see the deviation below).
- **Zero short options.** `-o` and `-h` are both retired
  (`30I:rul-spike-has-no-short-options`); `--help` is the only spelling.
- **One main book per target.** A second main-book operand — positional or
  `--book=` — is `cli-several-main-books`, refused pre-network. `Args::books:
  Vec<String>` is now `Args::book: Option<String>`, and `read_books`'
  newline-concatenation is deleted, which discharges the old §5 row 5.
- **`-` names stdin in any filename position** (`read_input`), and stdin is a
  COLLAPSED resource: `stdin_claimants` lists every lane that wants it — a `-`
  book, a `-` pre-source, `--results -`, and the DEFAULT claimants (the records
  lane on plan/apply/round-trip, `apply --host`'s artifact lane) — and two
  claimants refuse with `cli-stdin-claimed-twice`, naming both.

Two new codes, prose `[unwritten:]` per `error-authorship-tier`, with defining
cases at `crates/aid/tests/cli-several-main-books.loom` and
`crates/aid/tests/cli-stdin-claimed-twice.loom`.

Corpus: the e2e runner spells `--pre-source` in both seats (`shared_args` and
`round_trip_command`), and every committed replay line was respelled
mechanically. `hostsim::differential`, `spine_baseline`, `dorc-loom`'s replay
parsers and `dorc-coverage` were swept the same way.

**Not built, and why**: `rul-piped-stdout-implies-one-flat-plan` needs the three
emission modes to select BETWEEN, and there is exactly one emission form today, so
both halves are vacuous and its refusal has nothing to refuse. Building the
injected is-a-terminal seam now would be a seam with no consumer; it belongs with
step 7. Likewise `TargetedBook` is not minted: `Args`'s flat
`(book, pre_sources, host)` IS that shape at N=1, and the target-qualifier grammar
is explicitly unruled.

## §9 — Builder 2's deviation ledger (OPEN; for conductor/human adjudication)

None is self-endorsed.

1. **`dev-oracle-dir-survives-the-oracle-flags`** — `30I`'s ruling ledger names
   `-o`/`--oracle` DEAD and does not name `--oracle-dir`. It survives, now
   documented as "pre-source every `*.oracle.sh` in `<dir>`", and it is what
   `livetest.sh` and `render-baselines.sh` drive. If the intent was that the whole
   oracle-flag family goes, it is one rename (`--pre-source-dir`) plus those two
   scripts. Flagged rather than decided: renaming a flag two live scripts drive is
   not a builder's call.
2. **`dev-lint-keeps-its-oracle-flag`** — `dorc lint --oracle`/`--oracle-dir`
   survive (short `-o` retired). Lint LOADS nothing: the flag means "lint these
   files AS oracles", which `--pre-source` would misname. `30I` §2.4/§2.5 rule
   `plan`/`bundle` and say stream roles are per-subcommand, so the lint surface
   reads as out of scope — but the ruling text is not scoped explicitly.
3. **`dev-apply-plan-still-defaults-to-stdin`** — `owed-apply-takes-stdin-only-by-dash`
   is NOT discharged; `apply --host` without `--plan` still reads stdin. Instead
   that default is DECLARED as a stdin claimant, which is exactly the shape
   `30I` §2.5's own example describes ("a `-` book beside a stdin-defaulted
   `--plan`"). Making it `-`-only needs a "this mode requires this flag" refusal
   that has no code yet, and the brief calls the item punt-able.
4. **`dev-plan-dash-book-needs-explicit-results`** — because the records lane
   claims stdin by default, `dorc plan -` REFUSES unless `--results FILE` is
   given; `dorc probe -` is ordinary. That follows from §2.5's "never a silent
   precedence rule", but §2.4 lists `dorc plan -` as an ordinary spelling without
   saying it needs a companion flag. The alternative — letting a `-` book quietly
   win the stream from the records lane — is the precedence rule §2.5 forbids.
5. **`dev-builder-touched-help-prose`** — the usage synopsis and help page name
   flags, so retiring flags moved registry PROSE (`cli-usage-synopsis`,
   `cli-help-page`, and `aid-unloaded-sibling-oracle`'s help line). The edits are
   flag-token substitutions plus the one clause that said books CONCATENATE, which
   is now false. No new user-facing sentence was authored, and `-` is deliberately
   NOT documented in the help page — that would be authorship. Owed to the
   conductor: a `-` line in the options block, and prose for the two new codes.

## §10 — Builder 2: step 4 as built (the exact package sentinel)

### The parser keeps one name it used to throw away

`${x-}` and `${x:-}` lex to `WordPart::ParamComplex { empty_defaulted: Some("x") }`;
every other operator form answers `None` and stays as opaque as it was. Those two
bodies are CLOSED — the default is EMPTY — so no command substitution, arithmetic
or further expansion can hide in one, which is why carrying the NAME decodes
nothing and licenses nothing. No reader learns a VALUE.

That is the whole parser change, and it exists because the guard `30I` §2.2 spells
is nounset-safe and its variable's name was previously discarded. A bare `"$x"`
sentinel is deliberately NOT admitted: it aborts the loading shell under `set -u`,
so admitting it would bless an idiom the floor breaks on.

### The guard grew a second species, and the loader a decision

`oracle::load_inert::GuardCondition` is `CommandV { function }` |
`Value { name, literal, equals }`; `analysis::load::LoadCondition` is the loader's
copy. `[ "${x-}" = 'lit' ]` and `test`-spelled, either operand order, both senses.
`command -v` is untouched: same admission, same one-directional decision, same
conservative withhold (`30I:pin-command-v-load-model`).

`funcenv::sentinel_decides` is the recognition. Six conditions, each of which
withholds alone:

1. one branch loads exactly ONE target, the other is EMPTY;
2. the LOADING branch is the one taken when the sentinel does NOT match;
3. the target resolves exactly, from authored input, to a program the controller
   holds;
4. `sole_populator` — the target's own transitive closure assigns the tested
   variable, and NOTHING else in the authored world does, the book included;
5. `anything_removes` — no loadable program `unset -f`s a name that closure
   declares;
6. `sentinel_arm` — the ENVIRONMENT says which arm: every name the closure
   declares is bound to that closure's own definition (REUSE), or every one is
   Undefined (SOURCE). Mixed, ⊤, or live-from-another-unit withholds.

Condition 6 was a REPAIR mid-lane, and the reason is worth keeping: the first
implementation always took the source arm, which models a package RE-LOADING over
a regional shadow the reuse arm would have left standing — the licensing
direction, and exactly `30I` §13's "a book's own hand-written function" mislead.
Reading the arm off the environment costs the precision `command -v` had at
`load30-two-point-frames`'s regional load point (it decided TRUE there from a live
definition; the sentinel withholds because the live definition is the BOOK's) and
buys the correctness back. Pinned as
`a_package_name_shadowed_from_outside_withholds`.

The value plane is never asked what the sentinel HOLDS, and could not answer:
`analysis/src/value.rs` models an unset variable as absent-⊤, so "provably unset"
is unavailable there. **That is this lane's answer to `30Ia` §14's residual sizing
question — it is DISSOLVED rather than solved.** Recognition reads the idiom's
structure plus a NAME census (who could have populated the value at all), and the
environment for the arm; a value comparison never happens. The book-side half of
the census is `DefinitionTable::book_assigns`, a whole-AST NAME walk, because the
value plane cannot see an assignment it reads as ⊤ or one sited below the load.

### Minting moved to where a load is KNOWN to happen

`Loading::mints_speaker` is false inside an UNDECIDED guard's speculative branch
walks and sticky downward. So:

- an ordinary top-level `.` in a marked file mints, as before;
- a DECIDED guard's fallback `.` mints (the engine can say it runs);
- a RECOGNIZED sentinel guard mints `(sourcer, target)` explicitly, on BOTH arms —
  `30I` §3.4 case 2's "even when another package loaded the exact target first";
- an UNDECIDED guard's fallback mints NOTHING.

The last is a behaviour change from what builder 1 landed and from the literal
walk before it, both of which minted from any guard branch. It is the ruling
(`rul-speaker-minting-is-oracle-sourcing-only`, as `30I` §3.4 amends it), and
`cli::sourcing`'s `an_undecided_guard_mints_no_edge` /
`a_recognized_sentinel_guard_mints_its_edge` are the pair. Previously that hole
was contained only by the ⊤ the join produced — safety by composition across two
mechanisms, which `30Ia` §12 flagged as the shape to remove.

The reached-vouch-path `Must` half of `30I` §3.4 is deliberately NOT new code: it
is `HelperIndex::resolve` gating on the closures this edge feeds, plus the frame
lookup's `Must`-grade requirement. A same-named helper from another unit withholds
there, where it already did.

### The specimens

`load30-*` are re-spelled to the sentinel (directed work). Run sets are unchanged
and all three remain XFAIL; `floor30-dot-loader-function-errexit` is
byte-identical. `load30-subshell-errexit-fallback`'s guard is in the BOOK, so it is
ordinary book control flow either way — the re-spell keeps the specimen honest
about which idiom it exercises, and changes no engine answer.

## §11 — What step 4 did NOT land

1. **`rul-unannounced-cross-custody-fails-before-network` is NOT built.** The
   classifier already exists in substance — `oracle::closure::DenialReason::
   ResolvedOutsideCustody` IS `dependency-merely-happened-to-be-live`, and
   `explicitly-sourced` / `guarded-source-exact` are the two ways custody now
   reaches — but it SUSPENDS where the ruling wants a whole-run refusal. Design,
   ready to implement:
   - a cli-edge pre-pass beside `helper_conflict_diagnostics`, sited after the
     `HelperIndex` (which now sits below `funcenv::analyze`) and before the
     `probe`-mode early return: walk each non-book source's top-level role
     funcdefs, `closure_for(file, body)`, keep the `ResolvedOutsideCustody`
     denials;
   - mint one code (payload: the name, and the live declaration's `path:line`),
     prose `[unwritten:]`, spanned at the calling definition; a defining case like
     `cli-several-main-books`'s;
   - refuse the way the intake does — `report_at(...)` then
     `return Ok(RunOutcome::X)` BEFORE any artifact — with its own exit code,
     rather than `wrapper_incoherent`'s ship-then-signal shape;
   - `deliberate-external-utility` needs `closure::called_names` to stop counting
     the operand of `command <name>` as a reach. It currently counts both words;
     the fix is one arm in `walk`, and it must not also stop counting an ordinary
     call.
   Expected golden movement: `emit30-cross-custody-plural-helper-suspends` turns
   from "suspends, apply is the book" into "no mutation-authorizing plan", which
   is the movement builder 1 flagged and this builder inherited.
2. **`specimen-speaker-minting-is-observable` is NOT built**, and with it the
   mutation check the brief asks for. The finer-grained recognition cells ARE
   pinned at their ownership seats (`funcenv` TABLE 7, six cells;
   `oracle::load_inert` two; `cli::sourcing` two), so what is missing is the
   whole-product observation that minting has a RUN-SET consequence. Shape, as
   scoped: three `--pre-source` packages against one book — one recognized
   (converged verdict ⇒ its book site elides ⇒ absent from `expected.ran`), one
   whose sentinel a second unit also assigns, one whose helper a second unit also
   declares — the last two withholding, so their sites run. It needs a
   `probe-results.txt` matching what the mocked probe produces (gate-1 compares
   them), which is an iterate-until-green step.
