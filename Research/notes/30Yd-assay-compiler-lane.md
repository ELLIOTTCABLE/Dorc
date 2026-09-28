# 30Yd — the assay compiler lane: the first half of `30Y`, as built

> Tier: LLM-authored lane report (Opus builder, round 30, 2026-09-28), lane 1 of `notes/30Yc` § 3.
> Windows leg only. Confidence marks: `+SURE` observed, `~SUSPECT` inferred, `-GUESS` unchecked.
> Branch `ai/r30-assay`, based on `ai/main` at `9afc354c`.

## § 1-what-was-built-and-how-to-invoke-it

- **`mise run assay -- <spec.md> [--out <dir>]`** — `internal-tooling assay`
  (`spike/crates/internal-tooling/src/assay.rs`, `assay/alloy.rs`, `assay/sh.rs`). Reads the
  document's `alloy` and `sh` fences plus `shared.md` and `shared-laws.md` beside it (a missing half
  is empty), writes `assay.als shared.als species.als words.als claims.als laws.als
  book_corpus.als book_<name>.als` into one flat directory (default
  `<internal_tooling::target_dir()>/alloy/<spec-stem>/`), prints one JSON report on stdout. Exit 0,
  or 2 on a lint refusal (nothing written) or a usage error; 1 if writing fails. Relative paths
  resolve against `MISE_ORIGINAL_CWD`. It runs no solver and takes no lock.
- **`mise run alloy -- [caps] <file.als>...`** — now `preflight alloy`, then
  `internal-tooling exclusive --task alloy -- <the java invocation>`. New runner flags: `--heap <MB>`
  (2048, child `-Xmx`), `--procs <n>` (2, child `-XX:ActiveProcessorCount`), `--cpu <s>` (default
  the wall cap; polled once a second through `ProcessHandle.info().totalCpuDuration()`; a kill
  reports `timeout` with `exceeded Ns cpu`), `--batch-timeout <s>` (540; commands not started by
  then report `not-run`, exit 1); `--timeout` defaults to 120. One stderr line names the command
  count and every cap. A shutdown hook kills the live child when the runner exits (`-GUESS`: a
  hard `TerminateProcess` on Windows runs no hook; untested either way). A directory argument
  expands to its `*.als` files in sorted order, and a module whose only command is Alloy's
  synthesized `Default` (recognised by `Pos.UNKNOWN`; an authored `run Default` carries a real
  position, `+SURE` measured) yields no row.
- **`internal-tooling exclusive --task <name> -- <cmd> [args…]`** — the machine-global lock at
  `%LOCALAPPDATA%\dorc\heavy-work.lock` (unix: `$XDG_CACHE_HOME` or `~/.cache`, then
  `dorc/heavy-work.lock`), JSON `{pid, task, started, cwd}`, create-new. A live holder refuses with
  exit 75 and one stderr paragraph; a dead holder (`tasklist /FI "PID eq n" /NH`, `ps -p n`) is
  taken over with a stderr line. Released on the child's exit, success or failure. Only `alloy` is
  wrapped. Each platform leg has its own lock file (`+SURE`: WSL does not see `%LOCALAPPDATA%`).
- **`mise run preflight alloy`** — Workspace volume, 1 GiB disk (warm = cold), 3 GiB RAM.
- The glue a conductor uses by hand, from the out dir, in any shell: `mise run alloy -- .` (the
  defaults already bound it), or name modules to run a subset, e.g.
  `mise run alloy -- --timeout 60 book_corpus.als`. A document too large for one 540 s batch runs
  as several calls over module subsets.

## § 2-the-fixture-and-what-it-covers

`spike/crates/internal-tooling/tests/assay_fixture/` (`widgets.md`, `shared.md`, `shared-laws.md`,
`expected/*.als`, `negative/join_key_conflict.md`) and one test file, `tests/assay.rs`: the
fixture compiles byte-for-byte to `expected/` (a failure prints a line diff and the regeneration
command, `mise run assay -- spike/crates/internal-tooling/tests/assay_fixture/widgets.md --out
spike/crates/internal-tooling/tests/assay_fixture/expected`), and the negative document exits 2
with exactly one `join-key-coherence` finding and writes nothing. No JVM in any committed test.

Covered, each `+SURE` by reading `expected/`: two species under the shared claim base (`Wobble`,
`Jiggle`) and a world sig; three `one sig` claim atoms, one introducing `sprocket_heap`; a scoped
law with an unscoped premise twin (inherits the scope); an unscoped corpus check; `bookScope` in
`shared.md` with `Int` and no `seq` (assay adds `3 seq`) and `bookScope_twirls` in the document
with its own `seq` (left alone); a load file; a book loading it and a claim atom directly; a
fixture line with a map and a world fact that introduces `knob_word`, and no outcome (no `Line`
atom, `line_1` absent); a line with a `#=` claim declaration binding `this` and an outcome; a line
bracing `{gizmo}` with a trailing `for` override; redirections as single words; the append half's
law naming the document's `wobbly`.

Hand-verified once under Alloy (`+SURE`): `mise run alloy -- --timeout 60 --batch-timeout 300
shared.als laws.als book_corpus.als book_twirls.als` from `expected/`: 8 commands, every check
no-counterexample, every run sat, each under 1.2 s.

## § 3-the-strawman-3-smoke

`Research/notes/30Ya-strawman-3/spec/outcomes-past-a-wall.md` compiled with NO fence repair
(`+SURE`): 29 words (the hand build's 28 plus `assay_colon`), no classes, claim counts per book
17/22/22/27/27/27/25/23/23 exactly as the hand build's. Run in three foreground invocations
(`--timeout 60 --batch-timeout 540` each; § 5 `dev-smoke-split-into-three-runs`): 92 rows.

Verdicts: **no difference** (`+SURE`, compared row by row against `report.json` with the corpus
module renamed). The one red is `attributionByRemovalHonest` (counterexample), which `report.json`
records as expected; every other check no-counterexample, every run sat, every premise sat.
Slowest command 10.3 s. No `java` process survived any run (`tasklist`, `+SURE`).

After `9bf4a19e` added the corpus book's inhabitation run, `mise run alloy -- --timeout 60
--batch-timeout 300 book_corpus.als` over the recompiled strawman-3: `run book_corpus` is **sat**
(2.7 s, `+SURE`), so the all-claims-in-force universe is inhabited and the four corpus checks are
not green by contradiction; the four checks stayed no-counterexample.

| rows | report.json | assay | reading |
|---|---|---|---|
| shared | `bookScope` sat | `Default` sat | shape: the carrier is stripped, Alloy adds its default command |
| corpus (4) | module `corpus`, `exactly 28 Shword, exactly 17 Claim, exactly 0 Line` | module `book_corpus`, `exactly 29 Shword, exactly 0 Class, exactly 1 Line, exactly 17 Claim` | shape: the null-command line puts every claim in force; same verdicts |
| books (67) | `exactly 28 Shword` | `exactly 29 Shword, exactly 0 Class` | shape: `assay_colon` and the class bound |
| laws (20) | premise folded into the check's row | premise its own row | shape only |

Strawman-2 (`30Ya-strawman-2/spec/two-paths-one-inode.md`), compiled, not run: **refused** (`+SURE`),
37 `component-is-an-identifier` findings (map components that repeat the literal: `-c`,
`'%i %d'`, `g-w`, `g+w`, `/proc/sys/kernel/random/boot_id`) and one `braced-literal-is-named`
(`shared` braced `{bare_word}` and named nowhere). Not repaired: its content is frozen. `30Y` § 2.8's
own example uses that form (`#} stat -c '%i %d' a_path d_path`, with `dash_c`, `fmt_i_d`,
`g_minus_w` in its abridged words module), so § 2.8 and the brief's identifier rule disagree
(§ 6 `disagree-literal-components-in-maps`).

## § 4-where-alloy-the-runner-or-the-compiler-fought

- `chafe-alloy-adds-a-default-command` — **A.** A module with no command gets Alloy's `Default`
  run, so `shared.als` yields a meaningless row once assay strips `bookScope`. `~SUSPECT` harmless.
- `chafe-powershell-does-not-expand-the-book-glob` — **B.** `book_*.als` reaches the runner
  literally from PowerShell; the smoke ran from git bash. The runner or assay could expand it.
- `chafe-foreground-ceiling-below-the-batch-cap` — **B.** The agent tool's foreground ceiling is
  600 s, below the brief's 900 s batch cap, so a whole-document run cannot be one foreground call.
- `chafe-typos-fix-mode-rewrites-identifiers` — **B**, and the one that cost real work: `mise run
  fmt` runs typos in fix mode, which rewrote the dotless extension to `also` inside a test helper's
  name and inside the string literal of an extension compare, silently changing behaviour; the
  ignore rule covers only `.als` with its dot. Worked around with a `MODULE_SUFFIX = ".als"`
  constant. It also flags `tasklist`'s output-format flag, now dropped. Any future `.als`-handling
  code, and this note's own first draft, meets it.
- `chafe-main-was-clippy-red` — **B.** `ai/main` at `9afc354c` failed `clippy -D warnings` in
  `internal-tooling/src/corpus.rs` (`case_sensitive_file_extension_comparisons`, from `fa900486`),
  blocking this crate's lint; fixed in `fb74fa03` (§ 5).
- `chafe-lexer-is-private` — **B**, minor. `dorc_syntax` exposes only `parse`, so words come from
  walking the AST's command structure for spans; a substitution's inner spans are body-relative
  (`syntax/CLAUDE.md` `tn-coarse-subst-provenance`), so the walk never enters a word. A redirection
  node's span already covers its operator, fd, and target, so no gluing code was needed.
- `chafe-tests-label-not-in-gitlabels` — **B**, minor. The brief's `tests` label is not in
  `.gitlabels` (`test` is); the hook warned on `a862acf5`.
- No **C** entries: no spec content was changed, and strawman-3 needed no repair.

## § 5-deviations-from-the-brief (every one OPEN)

- `dev-fixed-main-clippy-lint` — `fb74fa03`: a one-line fix outside the remit (`corpus.rs`).
- `dev-two-extra-lints` — `f76b9cd8`: `map-line-follows-command` (a command with no `#}`, a second
  `#}`, a `#=` before any map) and `braced-literal-is-named` (a braced literal with no bare name has
  no atom to put in its class). Both are refusals the brief's rules imply but its lint list omits.
- `dev-stale-module-removal` — `f76b9cd8`: before writing, assay deletes `.als` files in the out dir
  that begin with its own generated header and that this compile did not produce, so a renamed book
  leaves nothing for `book_*.als` to pick up. Files without the header are never touched.
- `dev-scope-carrier-precedence` — `f76b9cd8`: a `run bookScope` in the document is also read, and
  wins over the shared one; the corpus book uses `bookScope` (there is no `bookScope_corpus`), and a
  corpus command carrying its own `for` keeps it, plus the exact bounds.
- `dev-no-corpus-run` — `f76b9cd8`: the corpus book gets no combined satisfiability `run`.
  REVERSED by the conductor (an unsatisfiable all-in-force universe greens every corpus check
  vacuously); built in `9bf4a19e` as `run book_corpus {}` at the corpus book's scope.
- `dev-scoped-runs-are-laws` — `f76b9cd8`: a scoped `run` that is not a premise twin goes to
  `laws.als`; an unscoped one to the corpus book.
- `dev-declaration-heads-and-parents` — `f76b9cd8`: a `#=` declaration is any sig head
  (`one`/`lone`/`some`/`abstract`/`var` too); `parent-is-a-known-sig` checks every sig head, with
  `univ`, `Int`, `String` accepted.
- `dev-smoke-split-into-three-runs` — no commit: three foreground runs at `--batch-timeout 540`
  instead of one at 900 (`chafe-foreground-ceiling-below-the-batch-cap`).
- `dev-expected-files-carry-the-generated-header-only` — `a862acf5`: the `.md` fixture files carry
  the "means nothing" line; the generated `expected/*.als` carry only assay's header, which names the
  fixture's path.
- `dev-liveness-unknown-refuses` — `ef3332a5`: a liveness check that cannot run answers "alive",
  so the lock refuses rather than risking two solvers; exit 127 when the wrapped command cannot
  spawn, 2 on usage or lock I/O.
- `req-typos-ignores-the-dotless-extension` — requested by the conductor, not a deviation:
  `a888048e` adds the bare word (also inside `_`-joined identifiers) to both typos configs; `mise
  run fmt` left scratch lines `als_files = "als"` untouched, and the old configs flagged them.
- `req-runner-bounds-under-the-harness-ceiling` — requested by the conductor, not a deviation:
  `d4cb169b` sets `--timeout 120` and `--batch-timeout 540` as defaults and adds the shutdown
  hook.
- `req-runner-directory-and-default-skip` — requested by the conductor, not a deviation:
  `d4cb169b` expands a directory argument and skips a module's sole synthesized `Default`.
- `dev-preflight-figures-not-load-measured` — `4334cefa`: 1 GiB and 3 GiB are the heap cap plus
  margin and the fixture's ~20 KiB of modules; no run was measured under memory pressure.

## § 6-where-this-build-and-30Y-disagree

- `disagree-literal-components-in-maps` — `30Y` § 2.8 writes literal components (`-c`, `g-w`) and
  shows munged words for them; the brief's rule makes every bare component an identifier, so that
  form refuses (strawman-2 is exactly this).
- `disagree-load-displacement` — `30Y` § 2.4 has a later load displace an earlier one answering the
  same thing; built as a dedupe only, per the brief's "union of the book's load sets".
- `disagree-harness-claim-abstract` — built to `30Y` § 2.2 (`abstract sig Claim`); strawman-3's
  hand harness is not abstract. The smoke's verdicts did not move (`+SURE`).
- `disagree-braced-only-literals` — `30Y` § 2.1 says any component may be either; a literal that is
  only ever braced has no word, so it refuses (`dev-two-extra-lints`).

## § 7-comment-count

`grep -c '^\s*//'` over the Rust added or changed: `assay.rs` 32 of 1154 lines, `assay/alloy.rs`
26/620, `assay/sh.rs` 13/275, `json.rs` 8/147, `exclusive.rs` 15/260, `tests/assay.rs` 9/124,
`preflight.rs` +7 added. About 4 percent overall.
