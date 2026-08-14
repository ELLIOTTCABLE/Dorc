# 28U (née 293) — the CLI input surface, and the role-function collision it exposes

Round scope: retire the input FLAGS in favour of positionals, make the dialect marker the
book/oracle discriminator, and make in-book role functions real. Two phases, landed
separately. Section 5 is not this round's work — it is a durable handoff for a focused
design sitting the human is opening separately.

Authority above this note: root `USER_STORY.md` / `KNOBS.md` (human-audited),
`spike/CLAUDE.md`, `spike/crates/{cli,oracle,syntax}/CLAUDE.md`.

## 1. What the human ruled (typed this sitting)

- `rul-named-files-are-positional` — named input files are bare positionals, so a shell
  glob is the ordinary way to pass many. `--book=` / `--book`, `-o` / `--oracle` are
  DELETED, not deprecated (`rul-strawman-formats-no-compat`: pre-user, no aliases).
- `rul-include-dir-is-flat-by-default` — the bulk-load flag is `-I` / `--include-dir`,
  FLAT, with a separate `--include-recursive` opt-in. Reasoning is the U-shape law
  ("match semantics fully, or break loudly and clearly; never a confusingly-similar
  middle"): every surveyed include/library/module-path convention — GCC `-I`, `ld -L`,
  `rustc -L`, `javac --class-path`, GHC `-i`, `protoc -I`, `shellcheck --source-path`,
  `ansible-lint --rules-dir`, Ansible `roles_path`, Puppet `modulepath`, Make `-I` — is
  a flat, one-level, name-driven lookup, and NONE recurses. Recursion is always either a
  target-discovery tool taking a directory positionally (`eslint src/`) or an explicitly
  separate toggle. `-L` was rejected on evidence: it means "compiled linkable artifacts,
  searched only when a `-lname` asks", and would read as a category error.

  The SHELL world sharpens this rather than contesting it. No shell spells
  directory-of-definitions as a CLI flag at all — POSIX `sh`'s option set has nothing
  path-shaped, and every shell-native mechanism (`FPATH` in ksh93/mksh, `fpath`/`FPATH`
  in zsh, `YASH_LOADPATH`, `fish_function_path`) is an ENV VAR, colon-separated,
  resolved BY NAME (function `foo` probes for a file named `foo`), never a directory
  scan and never recursive. Two consequences worth carrying:
  1. Our audience's one deeply-ingrained model for "a drop-in directory of definitions,
     no per-file registration" is the `*.d` idiom — `run-parts(8)` ("runs all the
     executable files named within constraints... Other files and directories are
     silently ignored", no `--recursive` option), `/etc/profile.d`,
     `/etc/bash_completion.d`, `cron.d`. It is unanimously SINGLE-LEVEL, and it is a
     packaging convention rather than a shell feature. A flag that reads as `*.d`-shaped
     and then silently recurses collides with the one convention this audience has
     memorised. This is the strongest argument for the flat default.
  2. If an env twin is ever wanted, the idiomatic shape for this audience is
     `<NOUN>PATH`, not `DORC_INCLUDE`. Note also bash 5.3 (2025) added `source -p PATH`
     — the shell world's newest instinct for "flag naming an override search location"
     reaches for `-p`/"path"; we should NOT borrow that, since `-p`/`$PATH` reads as
     "where executables live" to exactly these users.
  (Survey confidence: the dot-builtin PATH-search wording is confirmed from primary
  sources for POSIX, bash, zsh and yash; ~SUSPECT and unconfirmed for dash, posh, ksh93,
  mksh and busybox ash, where primary quotes could not be pulled. Nothing in this round
  depends on that gap.)
- `rul-you-named-the-library` — `24H:ack-6`'s "dorc never loads an oracle you did not
  name" is WIDENED: naming a library directory counts as naming its oracles. Rationale
  (human): nobody will enumerate 180 files from a GitHub repo of half-maintained
  oracle-shaped junk, and that repo is a real, in-scope ops artifact.
- `rul-marked-file-is-load-inert` — a marker-carrying file must be provably no-op to
  load. Top level may hold function definitions and BARE ASSIGNMENTS (file-global
  constants are a must-have); it may not hold commands.
- `rul-book-is-plain-sh-always` — already welded in the root docs this sitting
  (`USER_STORY.md` stage 4: "the book itself stays plain sh, always; dorc-lang and its
  marker live only in oracle files"; `KNOBS:kTYANNOT` carries the same sentence). This
  round implements it.
- `rul-one-authoritative-argument-parser` — three implementations of this grammar exist;
  there will be one.
- `rul-emission-keeps-file-provenance` — the analysis unit may stay unified (that is the
  expected default), but nothing downstream may LOSE which file a command came from:
  the target shape is N books -> N plans, eventually N -> (N x H).

## 2. Phase one — the flags, and the parser consolidation

Delete `--book=`/`--book`, `-o`/`-oPATH`/`--oracle`. Positionals classify by marker
content: marked ⇒ oracle-input, unmarked ⇒ book-input. `--oracle-dir` becomes
`-I`/`--include-dir` (flat) plus `--include-recursive`; both select by MARKER CONTENT,
not by the `*.oracle.sh` filename convention, because an upstream junk-repo will not
honour our naming and a marked file is self-describing (and, per
`rul-marked-file-is-load-inert`, provably inert). Unmarked `.sh` files under an include
dir are walked past and named in the loaded-source inventory.

Three parsers collapse to one. The survivor is `dorc_cli`'s lib target (already the
`289:rul-worldless-route-honest-trigger` seam). The two that go:

- `spike/crates/coverage/src/main.rs:60-99` — an independent hand-rolled copy,
  self-described as "cli parity"; drives `mise run coverage`.
- `spike/crates/dorc-loom/src/consumer.rs:943-986` (`parse_direct_plan`) — a shadow
  parser recognising ONLY `--book=PATH` and no oracle flag, driving 35 aid
  defining-cases in-process. Precedent for calling the real parser already exists in the
  same file (`fire_invocation_error`).

`crates/cli/CLAUDE.md`'s `lib-target-is-a-loom-seam` currently reads "nothing outside
`dorc-loom` and the two bins may depend on it"; it must be amended to name
`dorc-coverage` rather than quietly widened.

## 3. Phase two — the in-book lift, and per-file provenance

`USER_STORY.md` stage 3 shows a bare three-line `foobar__is_converged()` written INSIDE
the book, with "the function's *name* is the entire opt-in". That does nothing today:
`dorc_oracle::lift`, `predict::lift_predicts` and `verdict::VerdictSet::lift` all read
`oracle_refs` only (`cli/src/main.rs:742-757`); `book_src` is read afterwards and only
reaches `dorc_syntax::parse`. The stage-3 rung is unimplemented.

Phase two feeds every input file to the role lifts, so membership becomes what
`families-and-roles` already claims it is — name-derived, "never file, never author".

- **Ordering**: the book sorts LAST in the ordered source list. Under today's
  first-match-in-load-order resolution that makes the change strictly additive — a book
  function for a tool no loaded oracle covers wins (stage 3 works); a book function
  colliding with a loaded oracle stays inert exactly as it is today, so existing goldens
  do not move. Whether an admin's own book SHOULD win is section 5's question, not this
  round's.
- **Provenance**: `read_books` (`cli/src/main.rs:284-295`) concatenates books
  newline-joined and keeps only the first path as `book_name`, so a multi-book unit's
  line numbers are offsets into the concatenation. That already violates
  `AID-NEEDS:law-lineno-identity` ("one line-number space, the source file's,
  everywhere, round-trippable into `dorc why` addresses"). Fix: one `SourceFileId` space
  over EVERY input, book and oracle, subsuming today's `OracleFileId` (`core/src/lib.rs:89-96`
  — already "the index into the driver's ordered oracle-source list"), plus a per-book
  offset map so every span resolves to a real (file, line). Emission stays a single
  artifact; the fan-out becomes an emission change rather than an archaeology project.

## 4. The load-inert check

`spike/crates/oracle/CLAUDE.md` already carries `declarations-only-files` ("a top-level
mutator or unmodeled construct in an oracle file is a loud ⊤-reject"). Nothing
implements it. This round does.

Mechanically: walk `NodeKind::Script { items }` (`syntax/src/ast.rs:104-145`); permit
`FuncDef`, and `Simple` with EMPTY `words` (the AST's own spelling of a bare assignment,
per its doc comment). Reject everything else with a span.

Two edges worth stating because a naive shape-check misses them:

- `CERTS=$(hostname)` is an assignment with empty `words` that RUNS a command. Command
  substitution and arithmetic inside a top-level assignment value are rejected too.
- `export FOO=bar` / `readonly FOO=bar` are commands by AST shape and stay rejected for
  now, per `271:rul-posix-in-spirit-defaults` (conservative first; relaxing is cheap,
  tightening is not).

Fixture cost is near-zero: of 141 marker-carrying files in the tree, every
`crates/cli/tests/**/*.oracle.sh` is already definition-only. The whole newly-illegal
population is three synthetic single-file lint cases in `crates/aid/tests/` —
`unmodeled-wall-inventory.loom` (a book wearing an `oracle.sh` filename plus a marker),
`mark-rc-arity-exceeded.loom` and `mark-standalone-rc-consumer.loom` (top-level marks
outside any funcdef, exercising `lint_mark_subset`,
`oracle/src/predict/parser.rs:1616-1635`). `Research/notes/` strawmen are out of scope:
`26K` records "Strawmen books: REMAIN AS-IS (human-ruled: historical, explicitly
imagination-tier)".

## 5. DEFERRED, for a focused sitting — two definitions of one role function

<!-- superseded 2026-08-13 (human-typed ruling): conflicting contracted names in
     loaded scope FAIL-FAST, plan-time pre-network ONLY — never once probing begins;
     past that point the engine must best-effort through to some `plan` output.
     Start-simple; caveats/allowances may evolve. Rides the kernel work as
     `28Q:rul-name-collision-fails-fast`. The sitting below did not happen; §5c is
     settled toward flat-name-collision-fails-fast for now. -->

Human's framing, which the evidence supports: there must be a SINGLE function to
literally call at runtime; engine source-generation is ruled out
(`rul-ternary-verdict`: never engine-synthesized sh); merging two authors' argparses
would be gnarly even if generation were permitted. Suspicion is that the pattern below
is a non-starter.

### 5a. The pattern, and why it exists

Five case dirs ship two files that both define `apt_get__predict()`:
`headline-pi-webhost`, `headline-partial`, `headline-guarded-realistic`,
`exec-modeled-wall-runs` (all `package.oracle.sh` + `pkgindex.oracle.sh`), and
`guard23-reingest-collision-verbatim` (`book.sh` + `package.oracle.sh`).

It is deliberate, not sloppy. One command spans two state-kinds, described by two
separate kind-owners in two separately-publishable files:

```sh
# package.oracle.sh                     # pkgindex.oracle.sh
apt_get__predict() {                    apt_get__predict() {
   ... case $verb in                       ... case $verb in
      install) ... : sm.dorc.Package…         update) ... : sm.dorc.PkgIndex@fresh ;;
      purge)   ... :! sm.dorc.Package…     esac
   esac                                 }
}
```

The ecosystem cost of forbidding it outright: the package-oracle author and the
pkgindex-oracle author become one person editing one file, and two independent upstream
repos can never both describe `apt-get`.

### 5b. What the engine does today, and why it is fragile

Resolution is first-match-in-load-order, already `tc-`-flagged in place:
`analysis/src/effect.rs:395-398` ("if two checks both resolve, first-in-file-order wins
— flagged; no corpus case is ambiguous"), re-run at probe-compile in
`cli/src/main.rs:2001-2038`.

The emitted artifact does NOT contain one dispatching function. It REDEFINES the
function immediately before each call site — `headline-pi-webhost/expected.out` carries
two `apt_get__predict()` definitions, `headline-guarded-realistic` carries three:

```sh
# site 1: sm.dorc.PkgIndex@fresh
apt_get__predict() { verb=$1; shift; case $verb in update) test -n fresh ;; esac }
apt_get__predict 'update'; _rc=$?; ...
# site 2: sm.dorc.Package:nginx@installed
apt_get__predict() { ...install/purge arms... }
apt_get__predict 'install' '-y' 'nginx'; _rc=$?; ...
```

This is correct ONLY under strictly linear execution with each call directly beneath its
own definition. Named ways it breaks, none of them exotic:

- probe-phase parallelism, which is where the design says wall-clock is won
  (`spike/CLAUDE.md` perf-doctrine / no-reorder-ever);
- guard insertion into the APPLY artifact, where definitions and calls interleave with
  the book's own order rather than being emitted adjacently;
- any sourced-oracle idiom (`oracle/CLAUDE.md` R2-SHADOW already notes Dorc's own
  sourced-oracle idiom shadows `command -v`);
- any future emission that hoists definitions.

And it sits in direct tension with an existing refusal: `oracle/src/reserved.rs:10-14`
REFUSES munge-collisions precisely because "the shipped artifact would carry two
same-named funcdefs, last-writer-wins". The artifact carries two same-named funcdefs
right now, by a different route, and it is blessed.

### 5c. The options as they stand

- **flat-name-collision-fails-fast** — any duplicate role funcname across the input unit
  is a pre-network fail-fast. Human's stated lean. Costs the two-kind pattern and forces
  the four headline fixtures to merge.
- **ambiguity-fails-fast** — same-name definitions are legal; what is refused is two
  arms both MATCHING one book invocation. Cheap to build (the resolution loop already
  enumerates matches; stop taking the first, error on more than one), deletes the silent
  tie-break, preserves the two-kind pattern, and defers precedence. Does not by itself
  fix 5b's emission fragility.
- **one-function-per-provider-by-construction** — treat the two-kind case as a SPELLING
  error: one `apt_get__predict` dispatches every verb, and kind-ownership lives in the
  kinds rather than in duplicate funcdefs. Honest about the runtime constraint; costs
  the independent-publication story unless something else recovers it.

### 5d. Breadcrumbs for the sitting

- Whatever is decided must answer 5b's emission question, not only the lift question:
  what single text is shipped, and does it survive parallel probes and interleaved
  apply-time guards.
- `guard23-reingest-collision-verbatim` is the off-ramp-closure fixture and falls out of
  whatever is ruled: its `book.sh` is a stripped artifact carrying an inlined
  `apt_get__predict()` plus the stripped guard `apt_get__predict install -y nginx ||
  apt-get install -y nginx`, and the same dir also loads `package.oracle.sh`. Under any
  rule that lifts books, both definitions claim `install -y nginx`.
- Cross-check every candidate against `oracle/src/reserved.rs`'s munge-collision refusal;
  a rule that permits by one route what it refuses by another is the tell.
- Exclusion-check per root `AGENTS.md`: probe phase AND apply phase; admin AND engineer;
  reliable AND unreliable oracles.
- `KNOBS:kSILO` is the tension to reuse if the answer pushes correctness-code out of
  books into oracle libraries.

## 6. Open, needed before phase one lands

- `dorc why` takes an optional ADDRESS positional (`book.sh:N`, or free text to
  content-match), ruled by `289:rider-why-last-address-order` to be the first bare word
  wherever it sits. Once input files are also bare words, `dorc why webhost.sh
  cp.oracle.sh` reads `webhost.sh` as the address. `--book=`/`-o` used to disambiguate.
  Candidate resolutions: keep the first-bare-word rule; or let `why` take NO file
  positionals at all, since it answers from the receipt by default and the receipt
  already records the book path, the oracle set and their digests — which needs a story
  for `why --results=FILE`, the one form that drives a live analysis.

## 7. Churn inventory

- Parsers: `cli/src/lib.rs`, and the two consolidating callers in §2.
- argv construction: `internal-tooling/src/coverage.rs`, `hostsim/src/differential.rs`,
  `cli/tests/e2e.rs` (four sites; `round_trip_command` at :1197-1217 synthesizes block-0
  for every dir-form case, so the 84 case DIRECTORIES need no text edits).
- Looms: 35 of 62 in `crates/aid/tests/`, 25 of 29 in `crates/cli/tests/`. Block-0 lines
  regenerate; extra replay blocks (index >= 1, roughly a third of the cli looms carry
  2-4 apiece) are driven literally and need hand editing.
- Three aid cases lose or change their trigger: `cli-strip-got-a-flag` fires BECAUSE
  `--book=` is passed to `strip`; `cli-flag-needs-value` tests `--book` with no value;
  `cli-no-book-given` is the defining case for the premise that a book is required.
- Chrome: `crates/aid/tests/cli-help-page.loom` (loom-edited) and the hand-seeded
  `cli-usage-synopsis` row in `crates/aid/src/arrangement_lock.rs`. Prose is a
  conductor/human act (`27V:rul-error-authorship-tier`).
- Scripts and config: `spike/e2e/livetest.sh`, `spike/e2e/yardstick.sh`, `mise.toml`
  (prose only), `spike/crates/coverage/README.md`.
- `spike/docs/**` and every root doc carry ZERO occurrences of these flags — the docs
  tree's own law forbids anchoring prose to CLI spellings.
- `spike/fixtures/package.oracle.sh` is markerless yet uses dialect binds and marks; the
  inconsistency is latent today because no test runs the marker gate over it, and
  marker-driven classification may expose it.

## 8. Supersession owed

`24M:rul24M-typeless-floor` states "Share-a-file lives, marker-gated" and supersedes
24L's location-gating half. The root docs reversed that this sitting
(`rul-book-is-plain-sh-always`, §1). The note is historical and stays, but it carries a
human-typed ruling that now reads backwards to a trawling agent, so it earns an adjacent
supersession marker per root `AGENTS.md`.
