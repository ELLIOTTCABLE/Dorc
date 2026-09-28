# 30Ye — the assay lock-and-gates lane: the second half of `30Y`, as built

> Tier: LLM-authored lane report (Opus builder, round 30, 2026-09-28), lane 2 of `notes/30Yc` § 3
> and § 6. Confidence marks: `+SURE` observed, `~SUSPECT` inferred, `-GUESS` unchecked. Branch
> `ai/r30-assay`, based on `ai/main` at `f6a7deb3`. The brief was amended three times in flight by
> conductor message (`30Yc` § 6, last bullet): every bare component is a name, the translation
> co-change check is withdrawn, and the heavy tasks are wrapped in `exclusive`. This report is
> against the amended remit.

## § 1-what-was-built-and-how-to-invoke-it

- **`mise run assay -- <spec.md>... [--out <dir>] [--parse | --check | --write] [-- <alloy caps>]`.**
  Bare, it compiles as before. `--parse` compiles, then runs `mise run alloy -- --parse-only
  <out>` and refuses (exit 2, lint `alloy-parses`) on any module Alloy will not parse or
  typecheck. `--check` and `--write` parse the same way, then run `mise run alloy -- <caps> <out>`
  (caps after assay's own `--` pass through verbatim) and build one lock row per command;
  `--write` writes `<stem>.lock.json` beside the spec and exits 0, `--check` compares in both
  directions and exits 0 iff the run matches the lock row for row, result and premise included,
  green or red; a missing lock, a row either side lacks, or a moved result or premise exits 1.
  The reds a passing lock carries are listed in the report as `lock.accepted_reds` (`da374718`,
  after the human's ruling that reds fully acked by the committed lock pass). Exit 3 when the runner did
  not run at all (spawn failure, any exit but 0 or 1, including 75 contention, or no rows). The
  report per document carries `commands` (each lock row plus `wall_ms`, `solve_ms`, and the
  runner's message, which on a timeout is how far translation got) and `lock` (`path`, `status`
  of `written` · `matches` · `mismatch` · `missing` · `unreadable`, and `not_in_lock` /
  `not_in_run` as row objects). Several documents may be named; a `shared.md` or
  `shared-laws.md` path stands for every other document in its directory, and `<stem>.lock.json`
  for `<stem>.md`; the exit code is the worst over the documents (`spike/crates/internal-tooling/src/assay.rs`,
  `assay/lock.rs`, `assay/runner.rs`).
- **The lock file**: a JSON array, one row per line so its git diff is a row diff: `module`,
  `name`, `kind`, `scope` (Alloy's own rendering of the clause), `result`, `premise` on a check
  (the `<check>_premise` run's result; for a book line with no twin, its book's `run`, the corpus
  book's included; else `absent`), and `hash` (FNV-1a 64 over the command's
  generated text, LF-normalized, the same drift-alarm digest `spike/verify` uses). No timing and
  no translation size. `+SURE` measured on a scratch copy of the fixture: `--check` with no lock
  exits 1 `missing`; `--write` exits 0 and writes nine rows; `--check` exits 0 `matches`; one
  hand-flipped result exits 1 `mismatch` and names the row once in each direction. After
  `da374718`, the fixture plus one always-false law: `--write`, then `--check` exits 0 `matches`
  with that law in `accepted_reds`; five premises read `sat` (book lines and the corpus check),
  two `absent` (twinless laws).
- **`AlloyRunner.java --parse-only`**: one JVM, `CompUtil.parseEverything_fromFile` over each
  root, one row per file (`module`, `file`, `result` `parsed` or `error`, `message`), exit 0 or 1.
  Assay groups rows by message, so one error in a module every other opens (strawman-2's
  `shared.als`) is one finding naming the fourteen modules it stopped.
- **Words** (`assay/alloy.rs`, `atom` and `munge`): every bare map component is the name of the
  literal beneath it, and a literal only ever braced is named after itself; any name that is not
  an Alloy identifier becomes `munge(name)`. The munge in one sentence: ASCII letters and digits
  pass through, `_` doubles, every other character becomes `_<mnemonic>_` (`dash`, `slash`, `sq`,
  `pct`, `sp`, `gt`, `amp`, … or `u<hex>`), and a spelling that would not start with a letter or
  would be a keyword is prefixed `w_`; every escaped spelling has an even number of underscores
  and the prefix makes it odd, so the map is injective. `-c` → `w__dash_c`, `a-b` → `a_dash_b`,
  `443/tcp` → `w_443_slash_tcp`, `set` → `w_set`. The only refusals left on names are the join-key
  ones; `component-is-an-identifier` and `braced-literal-is-named` are deleted.
- **`internal-tooling exclusive`** now runs a command directly, without acquiring, when the
  inherited `DORC_HEAVY_WORK_HOLDER` names the pid the lock file names, and sets that variable on
  the command it runs when it acquires. The refusal opens `REFUSED, exit 75: CONTENTION, NOT A
  FAILURE. Nothing was checked and nothing is broken`. Wrapped in it (each as a hidden
  `<task>:held` twin holding the old body): `gate:full`, `gate:full-quiet`, `gate:arc`
  (`a746d9b9`); `bless`, `bless:dry`, `bless:case`, `bless:floor`, `verify:kani`, `verify:lean`,
  `verify:translate` (`edfb7f4e`); plus `alloy` as before. `bless:case`'s `BLESS=1` moved to its
  twin; the three `verify:*` keep `run_windows = "wsl -- mise run <task>"` on the outer task, so
  a Windows invocation delegates without holding the Windows lock and the WSL leg takes its own.
  `mise run bless:dry` on Windows, foreground: exit 0, 177 s wall-clock, the lock acquired and
  released (no lock file left).
- **hk steps**: `assay` in `linters` (pre-commit and check), glob `spec/**/*.md`, `mise run assay
  -- {{files}}`; `assay-lock` in `gates` (profile `slow`, builder completion), glob
  `spec/**/*.md` and `spec/**/*.lock.json`, `mise run assay -- --check {{files}}`. Two
  `step_globs` entries hold the pre-commit glob open (a spec path seen, a design note unseen).

## § 2-the-strawman-2-result

`Research/notes/30Ya-strawman-2/spec/two-paths-one-inode.md` now **compiles**, zero refusals
(`+SURE`; 37 before). 21 words, 15 of them literals; munged: `-c`, `'%i %d'`, `'%i %d %h'`,
`/proc/sys/kernel/random/boot_id`, `g-w`, `g+w`; braced-only `shared` is named `shared`. Its
`laws.als`, run alone at `--timeout 60`: one row, `error`, Alloy's syntax error at `shared.als`
line 51 column 100, the frozen shared half's `q.reader.before` (`before` is a reserved temporal
word in Alloy 6, the harness field is `above`). `--parse` refuses the whole document on that one
finding. No law row exists to record; the books were not run.

`~SUSPECT`: once that is repaired, the self-named word `shared` shares its name with the module
`shared` every generated module opens; whether Alloy resolves the two apart is unmeasured.

## § 3-gates-and-timing

- `mise run gate:step -- assay` and `-- assay-lock` over a throwaway `spec/x.md` (created and
  deleted uncommitted): the compile step green; the lock step red `missing`, then green `matches`
  after `--write` (`+SURE`). `hk check --why assay` selects one file; `hk check --why assay-lock
  --profile slow` selects the document and its lock. `mise run gate:floor` reported sanely with
  the spec present (24 checks selected) and after deleting it (20).
- Pre-commit step, warm, `hk run pre-commit --step assay spec/x.md`: 1.19 s, 1.21 s, 1.27 s over
  three runs; the bare `mise run assay` inside it is ~0.43 s (`+SURE`, Windows).
- `mise run test:hooks` green, the two new globs included.

## § 4-where-alloy-assay-or-the-tooling-fought

- `chafe-scope-error-surfaces-at-solve` — **A.** A document with no `bookScope` parses and
  typechecks clean, and Alloy refuses only when a command executes ("You must specify a scope for
  sig"), so the parse lint cannot catch it and it lands as an `error` row under `--check`.
- `chafe-scope-renders-int-lowercase` — **A**, cosmetic. Alloy's `Command.toString` renders `Int`
  as `int`, so a lock's `scope` column is Alloy's spelling, not the spec's.
- `chafe-main-slug-index-stale` — **B.** `ai/main` at `f6a7deb3` carried a stale `SLUGS.md`, and
  the `slugs` step's `*.md` glob fires on any Markdown commit, fixture documents included, so the
  first commit touching one refused. Regenerated in `6fe4b727`.
- `chafe-test-task-takes-one-filter` — **B.** `mise run test -- a b` dies in its doctest half
  (`cargo test --doc` takes one positional), so the hot loop ran `mise exec -- cargo nextest run
  -p internal-tooling`. A per-crate test task, or a filter the doc half can take, would serve.
- `chafe-too-many-lines` — **B**, minor. Clippy's line ceiling split the per-document driver in
  three; the pieces read fine.
- `chafe-out-of-repo-paths-verbatim` — **B**, minor. A spec outside the repo reports its lock
  path canonicalized (`\\?\C:\…` on Windows); only scratch copies meet it.
- No **C**: no specification content changed.

## § 5-deviations-from-the-brief (OPEN unless marked)

- `dev-exclusive-wrapping-partly-denied` — RESOLVED in `edfb7f4e`: the permission classifier
  denied the first attempt ("Modify Shared Resources"); the human approved the edit in
  manual-approval mode on the third round, and all seven are wrapped.
- `dev-verify-run-windows-stays-outer` — `edfb7f4e`: see § 1; the Windows side of the three
  WSL-only lanes delegates unlocked.
- `dev-held-twin-tasks` — `a746d9b9`: a wrapped task's body moves to a hidden `<task>:held` twin,
  because `exclusive` runs one command and a task body is a sequence; `preflight` therefore runs
  inside the lock for the three gates (`alloy` keeps it outside). `mise run <task>:held` bypasses
  the lock, visibly.
- `dev-several-documents-and-path-expansion` — `769445ed`: assay takes several documents (the hk
  steps hand it `{{files}}`), a shared half stands for its siblings and a lock for its document, and
  `--out` is refused with more than one document.
- `dev-hk-steps-through-mise-run` — `4e255561`: both steps call `mise run assay`, not
  `cargo run`, so relative paths resolve against hk's cwd rather than an outer mise's
  `MISE_ORIGINAL_CWD`. Costs ~0.3 s of the pre-commit figure above.
- `dev-two-steps-not-one` — `4e255561`: with the co-change check withdrawn the pre-commit step is
  the compile alone; the lock step is separate, in `gates`.
- `dev-step-globs-entries` — `4e255561`: two `Reach` rows added to the existing wiring battery.
- `dev-class-names-munged-too` — `cee65978`: a braced class that is not an identifier is munged
  like a name rather than refused, so no map component refuses at all.
- `dev-parse-findings-grouped` — `769445ed`: one `alloy-parses` finding per distinct message,
  naming the modules it stopped.
- `dev-json-key-read-skips-values` — `769445ed`: `json::read_str` now takes the first quoted key
  followed by a colon, so a value spelling a key name (a command named `result`) cannot shadow the
  real key; `exclusive`'s lock reader rides the same function.
- `dev-slugs-regenerated` — `6fe4b727`: see `chafe-main-slug-index-stale`.
- `dev-no-fixture-lock-committed` — no commit: the fixture's lock was exercised on a scratch copy
  only; a committed one would be bytes no test reads.
- `dev-contention-is-runner-failure` — `769445ed`: a nested `alloy` refused with 75 surfaces as
  assay's exit 3, the lock's stderr paragraph passing through, not as 75.
- `dev-translation-cochange-not-built` — withdrawn by the human in flight; nothing was built.

## § 6-where-the-build-and-30Y-disagree (against `ai/main` at `b1a03151`)

- `disagree-munge-injective-overall` — `30Y` § 2.1: "every name that is not already an Alloy
  identifier becomes one under a deterministic, injective munge". The munge is injective, but
  identity on identifiers plus munge on the rest is not: `-c` and an authored `w__dash_c` meet.
  Built as the join-key refusal § 2.1 already names; the negative fixture pins it.
- `disagree-book-checks-premise-absent` — RESOLVED in `da374718`: `30Y` § 2.5 item 3 now makes
  a book's run its lines' witness, and the lock records that run's result as their premise.
- `disagree-locked-red-still-exits-one` — RESOLVED in `da374718` by the human's ruling: a set of
  reds fully acked by the committed lock is a pass; see § 1.
- `disagree-result-vocabulary` — the lock's `result` also takes the runner's `error` and
  `not-run`, which § 2.7's list omits.

## § 7-comment-count

Added lines across the Rust changed (`git diff f6a7deb3 HEAD`, `grep '^+\s*//'`): 78 of 931,
8.4 percent. Per file, comments/added: `assay.rs` 18/297, `assay/alloy.rs` 13/120,
`assay/lock.rs` 19/254, `assay/runner.rs` 11/92, `assay/sh.rs` 2/13, `exclusive.rs` 6/55,
`json.rs` 3/31, `step_globs.rs` 2/14, `tests/assay.rs` 4/55.
