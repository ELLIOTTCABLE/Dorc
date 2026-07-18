# 27Ra — dorc-lint prior-art digest (light sweep, ungraded)

AI-gathered (Opus web-gatherer under the Fable conductor, 2026-07-18; 15 fetches).
Sibling of `27R` (the plan-of-record; its §8 carries the adopted deltas). This is a
LIGHT sweep, not an interactive-research graded round: per-claim certainty markers
ride inline (+SURE/~SUSPECT/-GUESS), source URLs per claim, no source-ID grading.

---

# Prior-art for a `lint`/`doctor` subcommand — stealable patterns

Research digest, 2026-07-18. Every claim carries its source URL. Certainty markers: +SURE (from primary docs/source read directly), ~SUSPECT (from search snippets or partial fetch), -GUESS (inference / general knowledge not re-confirmed this session).

---

## Area 1 — "doctor" subcommand UX

### `brew doctor` (the gold standard for warning-fatigue handling)
Source: https://raw.githubusercontent.com/Homebrew/brew/master/Library/Homebrew/cmd/doctor.rb (source read directly)

- +SURE Exit contract: "Will exit with a non-zero status if any potential problems are found." Sets `Homebrew.failed = true` on the first finding.
- +SURE The famous anti-warning-fatigue preamble, printed **once, to stderr, before the first warning only** (`first_warning` flag, suppressed under `--quiet`):
  > Please note that these warnings are just used to help the Homebrew maintainers with debugging if you file an issue. If everything you use Homebrew for is working fine: please don't worry or file an issue; just ignore this. Thanks!
- +SURE Clean-state message: prints `Your system is ready to brew.` when nothing failed. On failure it prints a `support_tier_message(tier:)` instead.
- +SURE Checks are individually named methods (`check_*`) and **pluggable/runnable in isolation**: `--list-checks` lists all audit methods; passing names as args runs only those. Slow checks (`check_for_broken_symlinks`, `check_missing_deps`) are deliberately ordered last.
- +SURE Severity tiers exist: each finding has a `:tier`; the overall tier is `max_by { |f| f[:tier] }`. There is a hidden `--json` mode emitting `{ tier:, findings: [...] }` (machine mode co-exists with human mode in the same command).
- ~SUSPECT Positioned as a *pre-issue-filing* diagnostic, not a daily tool: Homebrew's Troubleshooting doc says "Run brew update twice and brew doctor (and fix all the warnings) before creating an issue!" https://docs.brew.sh/Troubleshooting ; SO answer: "you can safely ignore these warnings if everything you use Homebrew for works fine." https://stackoverflow.com/questions/44734735/brew-doctor-warnings-delete-or-ignore

### `flutter doctor`
Source: https://github.com/Dart-Code/Dart-Code/issues/852 (exit-code snippet); https://www.codecademy.com/article/check-your-flutter-installation-with-flutter-doctor (thin)

- ~SUSPECT Output is grouped into categories (Flutter, Android toolchain, Xcode, Chrome, Connected device…) each prefixed with a status marker; summary line "Doctor found issues in N categories." or "No issues found!". `-v` adds per-check detail. Exact `[✓]`/`[✗]`/`[!]` markers are -GUESS (general knowledge; codecademy text didn't quote them).
- ~SUSPECT **Anti-pattern for CI:** `flutter doctor` exits **0 even when it found issues** ("Doctor found issues in 1 category. exit code 0"). Cannot be used as a CI gate. Contrast brew's non-zero exit.

### `npm doctor`
Source: https://docs.npmjs.com/cli/v10/commands/npm-doctor

- +SURE Runs ~7 named checks: registry connectivity, npm version, node version, configured registry, git-in-PATH, permissions (cache + node_modules writability), cached-package checksum validation.
- +SURE Checks are selectable: "By default npm runs all of these checks. You can limit what checks are ran by specifying them as extra arguments."
- -GUESS Output is a table with `Ok`/`not ok` per row (docs didn't confirm exact columns; general knowledge says `Check | Value | Recommendation`).

### `mise doctor` — not fetched this session. -GUESS: sectioned health report similar to the above.

---

## Area 2 — shellcheck as a mechanized dependency
Source (all +SURE): https://www.mankier.com/1/shellcheck (full manpage read); https://github.com/koalaman/shellcheck (README, stability note)

- **Output formats** (`-f`/`--format`): `tty` (default, human), `gcc`, `checkstyle`, `diff`, `json`, `json1`, `quiet`.
  - `gcc`: `<file>:<line>:<column>: <type>: <message>` — the simplest stable machine-parseable line format; editor-friendly.
  - `json1`: compact, documented as the intended machine interface. Shape: `{ comments: [ { file, line, column, level, code, message }, ... ] }`. **Tabs counted as 1 char.**
  - `json`: explicitly a **legacy** format — raw array of comments, tab stop of 8. json1 is the one to target for new integrations.
  - `checkstyle`: XML, each error carries `source='ShellCheck.SC####'` (the stable rule code).
  - `diff`: unified-diff autofixes, pipe to `git apply`/`patch -p1`.
  - `quiet`: no output, exit 0/1 only, stops at first issue.
- **Exit codes** (Return Values section): `0` all clean; `1` scanned OK but issues found; `2` some files couldn't be processed (not found etc.); `3` bad invocation syntax (unknown flag); `4` bad options (unknown formatter). → clean/findings/operational split, plus two distinct invocation-error codes.
- **Severity gating**: `-S`/`--severity error|warning|info|style` sets the *minimum* severity to report (default `style`, i.e. everything).
- **Dialect selection**: `-s sh|bash|dash|ksh|busybox`. `-s sh` = POSIX sh (not the system's) and **warns of portability issues** — overlaps checkbashisms. Deduced from `shell` directive, shebang, or extension if not given.
- **Inline directives** (pass-through concern for Dorc): `# shellcheck disable=SC2035` (comma-separated, range `disable=SC3000-SC4000`, `disable=all`). File-wide if before the first command; else applies to the next command/compound-block. Other keys: `enable`, `source=`, `source-path=`, `shell=`, `external-sources=`, `extended-analysis=`. `.shellcheckrc` and `SHELLCHECK_OPTS` env var carry defaults.
- **Compatibility promise**: README — new versions publish new warnings; pinning severity/version "avoids any surprise build breaks when a new version with new warnings is published." So the *codes* (SCxxxx) are stable identifiers; the *set of warnings emitted* grows over time. Codes never get reused.

---

## Area 3 — checkbashisms
Source (all +SURE): https://manpages.debian.org/testing/devscripts/checkbashisms.1.en.html (manpage read directly)

- **Default output shape** (confirmed via multiple real outputs, e.g. https://forum.endeavouros.com/t/.../19319 , https://lists.linux.it/pipermail/ltp/2021-September/024869.html):
  `possible bashism in FILE line NN (explanation): <the offending source line>`
  e.g. `possible bashism in foo.sh line 7 (type): type foo`
- **Linter mode** `--lint`/`-l` (for editor integration) emits a *different, more standard* shape:
  `{filename}:{lineno}:1: warning: possible bashism; {explanation}` (column is always 1).
- **Exit values are ADDITIVE (bitwise-OR-ish sum)**: `0` clean; `+1` a possible bashism was detected; `+2` a file was skipped (unreadable/not found); `+4` no bashisms detected in a *bash* script. So a run can return e.g. `3` = bashism found (1) AND a file skipped (2). Callers must mask bits, not `== 1`.
- Flags: `--posix`/`-p` (check Debian-Policy-required-but-non-POSIX, implies `-n`), `--newline`/`-n` (`echo -n`), `--force`/`-f` (check even non-sh shebangs), `--early-fail`/`-e` (stop at first error).
- ~SUSPECT Output format has been stable for years (the `possible bashism in ... line NN (...)` shape appears unchanged across many years of kernel/LTP/Arrow logs). It is a Perl script from devscripts; no JSON/machine format exists — you must parse the text.

---

## Area 4 — multi-linter wrappers / aggregators (the normalization schemas)

### golangci-lint
Sources: exit codes https://pkg.go.dev/github.com/golangci/golangci-lint/pkg/exitcodes (+SURE, source-of-truth constants); formats+severity https://golangci-lint.run/docs/configuration/file/ (+SURE)

- **Exit codes (granular, one per operational failure class)**: `Success=0, IssuesFound=1, WarningInTest=2, Failure=3, Timeout=4, NoGoFiles=5, NoConfigFileDetected=6, ErrorWasLogged=7`. Steal this: distinct codes for "found problems" vs each kind of "couldn't run."
- **Decouple exit from findings**: `--issues-exit-code=0` makes findings non-fatal while keeping the report (from https://www.reddit.com/r/golang/comments/m2anzj/ ). Severity level is *independent* of exit code.
- **Output formats** (`output.formats` config, v2): `text`, `json`, `tab`, `html`, `checkstyle`, `code-climate`, `junit-xml`, `teamcity`, `sarif`. Multiple can be emitted **simultaneously**, each to its own path (stdout/stderr/file).
- **Severity normalization**: `severity.default` sets a baseline; the sentinel **`@linter` preserves each linter's own native severity**; `severity.rules` (same matcher syntax as exclusion rules) remaps per-linter → severity level. This is the "map foreign findings into one schema" pattern.

### reviewdog (the "parse anything, filter by diff" model)
Source: https://github.com/reviewdog/reviewdog (+SURE via WebFetch)

- **Generic parser = errorformat** (Vim's errorformat, "scan-f like"): default `%f:%l:%c: %m` → `{file}:{line}:{col}: {message}`. Specifiers `%f %l %c %m %%`. Supplied via `-efm`, or use a predefined name from `reviewdog -list`. This is how it ingests *any* linter's plain text without a bespoke adapter.
- **Native rich schema = RDFormat** (`rdjson` / `rdjsonl`, proto at proto/rdf): supports multiline ranged comments, `severity`, rule `code` with URL, and code `suggestions`. `rdjsonl` = one JSON object per line (streamable).
- `-f` accepts: `rdjson`, `checkstyle`, `sarif`, `diff`, and named formats (`golint`, etc.). So it also natively consumes checkstyle/SARIF — i.e. emitting checkstyle or SARIF gets you reviewdog for free.
- **Diff-aware filter modes** (`-filter-mode`): `added` (default — only added/modified lines), `diff_context` (changed lines ±N), `file` (any finding in a touched file even if not on a changed line), `nofilter` (everything). `-diff="git diff FETCH_HEAD"` supplies the diff.
- **Exit contract**: default `-fail-level=none` → exit 0 even with findings. `-fail-level=[any|info|warning|error]` → exit 1 if any finding ≥ that severity. `-fail-on-error` catches all results.
- Severity `info`/`warning`/`error`; GitHub Checks maps info→neutral, warning→neutral, error→failure.

### MegaLinter / treefmt / pre-commit (tool-availability handling)
Sources: https://megalinter.io/latest/ ; https://github.com/numtide/treefmt ; https://github.com/oxsecurity/megalinter/blob/main/.pre-commit-hooks.yaml (search-level only)

- ~SUSPECT MegaLinter aggregates 48–50 languages / dozens of tools behind a single entry point; ships as a container so all sub-linters are pre-provisioned (sidesteps "is the tool on PATH?" by bundling).
- ~SUSPECT treefmt is a *formatter multiplexer*: it expects each formatter to already be on PATH and runs them in parallel over matching files; config declares which command handles which glob.
- -GUESS pre-commit's model is the inverse: it *provisions* each tool in an isolated pinned env (its own venv/gem/node install), so availability is guaranteed by the framework rather than assumed. Relevant contrast for Dorc's "linter present on PATH?" decision.

---

## Area 5 — machine output formats & stable-code conventions

### rustc/cargo JSON (best-documented stability policy)
Source: https://doc.rust-lang.org/rustc/json.html (+SURE)

- **JSONL**: "JSON messages are emitted one per line to stderr." Each has a `$message_type` discriminator field (`"diagnostic"`, etc.) — parse by type.
- **Stable code vs unstable text**: `code.code` is "a unique string identifying which diagnostic triggered" (the stable machine key, e.g. `unused_variables`); `message` is human text (unstable). `code.explanation` optional.
- `level`: `error | warning | note | help | failure-note | "error: internal compiler error"`.
- `spans[]`: `file_name`, `byte_start`/`byte_end` (0-based), `line_start`/`line_end` (1-based), `column_start`/`column_end` (1-based), `is_primary`, `text[]` (source lines w/ highlight ranges), `label`, `suggested_replacement`, and `suggestion_applicability` ∈ `MachineApplicable | MaybeIncorrect | HasPlaceholders | Unspecified` — an explicit *confidence tier on autofixes*, so tools know which fixes are safe to apply automatically.
- **Forward-compat policy (quotable)**: "care should be taken to be forwards-compatible with future changes to the format. Optional values may be `null`. New fields may be added. Enumerated fields like 'level' or 'suggestion_applicability' may add new values." → the contract is "additive-only + tolerate nulls + tolerate new enum variants," NOT "frozen schema."

### eslint JSON conventions
Source: https://eslint.org/docs/latest/extend/custom-processors (+SURE, LintMessage type)

- `severity: 0 | 1 | 2` (off/warn/error — numeric, stable); `ruleId: string | null` (null when not rule-attributable, e.g. parse errors); `line`/`column`/`endLine`/`endColumn` all **1-based**; `fix: { range: [start,end], text }`; `suggestions[]`.

### SARIF
Source: https://www.sonarsource.com/resources/library/sarif/ (+SURE prose); spec https://docs.oasis-open.org/sarif/sarif/v2.0/csprd01/sarif-v2.0-csprd01.html

- OASIS standard, JSON. Minimal spine: `version`, `runs[]`, each run has `tool.driver` (+ `rules[]`), `results[]`; each result has `ruleId`, `level`, `message`, `locations[].physicalLocation.{artifactLocation, region}` (region = line/col span), plus `artifacts[]`.
- **Heavy/verbose**: "detailed, supporting deep nesting of artifacts, tool-provided metadata, and explicit links between issues and remediation resources." No official "minimal profile" exists (~SUSPECT — the sonarsource guide makes no mention of one), but in practice a *viable minimal subset* is `version + runs[0].tool.driver.name + runs[0].results[]` with `ruleId/level/message/one physicalLocation`. GitHub code scanning is the dominant consumer; producing SARIF buys GitHub PR annotations.

### Exit-code convention across the ecosystem (+SURE, synthesized)
The near-universal split, confirmed across shellcheck (0/1/2, +3/4 for bad invocation), checkbashisms (additive 1/2/4), golangci-lint (0/1 + 2–7 operational): **0 = clean, 1 = findings, ≥2 = operational/tool error**. Keeping "found lint" (1) distinct from "the linter itself broke" (2) is the load-bearing convention — a CI gate treats 1 as "fail the build" but 2 as "the check is broken, investigate."

---

## Area 6 — linting a TRANSFORMED source and mapping locations back

### ESLint processors (the canonical precedent — directly applicable to Dorc's strip-annotations-then-remap)
Source: https://eslint.org/docs/latest/extend/custom-processors (+SURE)

- `preprocess(text, filename)` → returns an array of `{ text, filename }` code blocks (the *transformed* source; can split one file into many virtual blocks). Each block is linted separately but attributed to the original filename. The block `filename` extension tells ESLint which config/rules apply.
- `postprocess(messages, filename)` receives a **2-D array** `Message[][]` (one inner array per preprocessed block) and **MUST**: (a) "adjust the locations of all errors to correspond to locations in the original, unprocessed code," and (b) flatten to a single 1-D array. This is exactly Dorc's remap step.
- **Autofix remapping is the documented hazard**: "By default, ESLint does not perform autofixes when a custom processor is used, even when `--fix` is enabled." To opt in you must additionally transform each message's `fix.range` (indices into the *processed* text) back to indices in the *original* file, then set `supportsAutofix: true`. The default-off is a deliberate admission that fix-range remapping is error-prone when the transform isn't a clean bijection.
- Vue/Svelte plugins (`eslint-plugin-vue` uses `vue-eslint-parser`, Svelte similarly) follow the same extract-block-then-remap model for `<script>` blocks. (-GUESS on internals; not fetched.)

### Documented / known failure modes when the map is approximate
- +SURE (implied by the autofix default-off above): non-bijective transforms make **fix/replacement ranges unsafe** — the primary reason ESLint disables autofix under processors by default.
- ~SUSPECT Inline suppression directives written in the *original* file (e.g. a `# shellcheck disable=` comment) may be stripped or shifted by the transform and fail to suppress — directives don't survive an annotation-stripping pass unless explicitly preserved. (Directly relevant: Dorc cannot copy comment-directives, and this is the exact class of bug.)
- -GUESS Off-by-one at block boundaries and multi-line spans crossing a stripped region are the classic remap bugs; sourcemap-based remappers (Babel/TS) mitigate with per-token maps rather than per-line offsets. Not confirmed with a fetched source this session.

---

## Area 7 — hot-loop friendliness (keep the authoring loop unbroken)

### ESLint `--cache`
Source: https://charpeni.com/blog/speeding-up-eslint-even-on-ci (+SURE)

- Cache file `.eslintcache` (default), relocatable via `--cache-location`. Contains **absolute paths** → user-specific, do not commit.
- `--cache-strategy`: `metadata` (default — file size + mtime; fast locally but **wrong in CI because git doesn't preserve mtime**) vs `content` (checksums file contents; the correct choice for CI). Steal: pick strategy by environment.
- Gotcha: **omitting `--cache` deletes the cache**; must be applied consistently (incl. pre-commit hooks). Must invalidate when the ESLint config / lockfile changes.
- Numbers: ~5 min → ~10–30 s on CI via caching.

### golangci-lint / general
- ~SUSPECT Parallel by default across linters + build/analysis caching (`golangci-lint cache` subcommand exists per CLI help https://golangci-lint.run/docs/configuration/cli/ ). Warm-cache CI pipelines cut from ~1 min to <30 s (https://wintermutecore.com/posts/go-ci-lint-pipeline-optimisation/ ).

### Tool-missing degradation (warn vs fail; opt-in strict for CI)
- +SURE brew's design: warnings are advisory, printed once with an explicit "just ignore this" escape hatch; the *diagnostic* is separated from *breakage*. Users are told to only care when something's actually broken. This is the antidote to warning-fatigue → tools people stop running.
- Pattern to steal: **default = degrade gracefully** (missing `shellcheck`/`checkbashisms` on PATH → warn/skip that engine, still succeed), **opt-in `--strict`/CI mode = hard-fail** if an expected engine is absent (so CI can't silently stop checking). reviewdog encodes the same idea via `-fail-level` (advisory by default, gating on demand).

### "Noisy doctor tools get ignored" (wisdom, soft sources)
- ~SUSPECT brew doctor is explicitly framed as run-before-you-file-an-issue, and the community consensus (SO, Homebrew discussions) is "safely ignore unless broken" — i.e. even the canonical example concedes its output is mostly noise and pre-empts it with the "just ignore this" preamble rather than trying to make every warning actionable. Sources: https://stackoverflow.com/questions/44734735/brew-doctor-warnings-delete-or-ignore ; https://github.com/orgs/Homebrew/discussions/4280 ; https://docs.brew.sh/Troubleshooting

---

## PATTERNS WORTH STEALING (consolidated)
1. (A1) One-time "these are advisory, ignore unless something's actually broken" preamble before the first warning — brew's exact device against warning-fatigue.
2. (A1) Distinct clean-state sentence ("Your system is ready to brew.") so success is unambiguous, not just absence of output.
3. (A1/A4) One command, two modes co-resident: human `tty` default + hidden/opt-in `--json` emitting the same findings with a `tier`/severity — don't split into two commands.
4. (A1) Pluggable named checks with `--list-checks` and "pass names as args to run a subset"; order slow checks last.
5. (A2/A5) Target a *compact JSON* (json1-style) as the stable interface; keep a legacy format frozen rather than mutating the live one.
6. (A2/A5) Stable rule **codes** (SCxxxx / `unused_variables`) are the contract; message text is explicitly unstable and codes are never reused.
7. (A2/A3/A4/A5) Exit-code trichotomy: 0 clean / 1 findings / ≥2 operational-error — and keep "linter broke" distinct from "found lint."
8. (A4) golangci-lint's granular exit codes (one per operational failure class) + `--issues-exit-code` to decouple exit from findings.
9. (A4) reviewdog errorformat (`%f:%l:%c: %m`) as a zero-adapter way to ingest arbitrary linter text; and emit checkstyle/SARIF to get reviewdog + GitHub annotations for free.
10. (A4) Severity normalization with a `@linter` sentinel that *preserves* each engine's native severity, plus rule-based remap — the schema-mapping pattern.
11. (A5) rustc's forward-compat policy: additive-only fields, tolerate null, tolerate new enum values — the stability promise for a machine format.
12. (A5) Per-fix confidence tier (`suggestion_applicability`: MachineApplicable/MaybeIncorrect/…) so consumers know which autofixes are safe.
13. (A6) ESLint processor contract: `preprocess`→virtual blocks, `postprocess` MUST remap locations to the original and flatten — the exact shape of Dorc's transform+remap.
14. (A7) `--cache` with a metadata-vs-content strategy switch chosen by environment (mtime local, content in CI).
15. (A7) Graceful-degrade by default when a third-party linter is missing; opt-in `--strict`/CI mode to hard-fail on absence (so CI can't silently stop checking).

## ANTI-PATTERNS TO AVOID
1. (A1) `flutter doctor` exits **0 even with issues** — unusable as a CI gate. Always let CI mode exit non-zero on findings.
2. (A3) checkbashisms' **additive exit codes** (1+2+4 summed) force bitmask parsing and are easy to misread as `== 1`. Prefer disjoint codes.
3. (A3) checkbashisms has **no machine format** — consumers must regex the prose `possible bashism in FILE line NN (...)`. Don't ship a tool whose only output is unparseable prose.
4. (A6) Remapping fixes/replacement ranges across a non-bijective transform is unsafe — ESLint disables autofix under processors by *default* for this reason. Treat fix-remap as opt-in and suspect.
5. (A7/A1) A doctor that emits many non-actionable warnings trains users to ignore it entirely (even brew concedes this and pre-empts with "just ignore"). Every emitted line should be actionable or explicitly flagged advisory.
