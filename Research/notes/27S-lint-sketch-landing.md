# 27S — `dorc lint` sketch: as-built landing note

AI-authored (Opus implementor under the Fable conductor, 2026-07-18, worktree
`agent-a9d8c7f25debbb150`, branch `ai/r27-lint-build` off `1d0590f`). Sibling of the
plan-of-record `27R` (its §§0-8b are the charter) + the prior-art digest `27Ra`. This note
is the as-built ledger, the rung-oracle-solo reconnaissance strain-note, deviations, seams,
and the tc-* flags. Append-only; never edited in place.

## §1 — As-built ledger (what landed where)

Three granular commits on `ai/r27-lint-build` (tip **ce69921** at writing):

- **strip line-map** (`crates/oracle/src/strip.rs`) — a new `strip_file_with_map()` +
  `StripMapped { text, line_map }`, kernel-pure (`strip-is-pure-erasure`). `strip_file` and
  the mapped variant now share `collect_strip_edits()`/`apply_edits()`, so the stripped BYTES
  are provably byte-identical to before (the pre-existing 6 strip tests + all e2e stayed
  green). The map is DERIVED from the same edits: an original line vanishes iff a
  deletion-edit (empty replacement) covers its content-start through its terminating newline
  — the whole-deleted marker line + bare-mark (`invariant:`) lines are the only shifts;
  within-line edits (bind rewrite, trailing-mark delete, shebang rewrite) keep their line.
  +SURE this captures every line-shifting case the corpus strip produces.
- **`crates/lint`** (new workspace crate `dorc-lint`) — the machinery (`27R` §1):
  - `finding.rs` — the ONE model: `Finding {path, line, col, severity, source, code, message,
    remap}` + `LintSeverity {Error,Warn,Info}` + `RemapFidelity {Exact,Approximate,None}` +
    `Coverage`/`SourceCoverage`/`SourceStatus` + `LintReport` (with `count_at_or_above` /
    `severity_counts` for the exit trichotomy). Sort key `(path, line, source, code)`.
  - `runner.rs` — the `ExternalToolRunner` DI seam (`available` + `run bytes→{rc,stdout,stderr}`)
    + `ToolRun` + a `NoToolsRunner`.
  - `source.rs` — the dumb registry: `LintSource` trait + `LintContext`/`LintInput`/
    `LintOptions`/`Rung` + `registry() -> Vec<Box<dyn LintSource>>`.
  - `source_analysis.rs` (rung-book) — parse→cfg over each file, Carrier diags → findings.
  - `source_unmodeled.rs` (rung-book) — per-book ⊤-wall inventory (count + first-wall line +
    approximate downstream-leaf count).
  - `source_verdict.rs` (rung-file) — verdict-body terminal-pipeline lint over `ctx.oracles`.
  - `source_external.rs` (rung-file) — shellcheck + checkbashisms adapters, the §4 degradation
    ladder (json1 → tolerant text → raw), rc-zero/nonzero-only, absent-is-info, remap via the
    strip line-map.
  - `json.rs` — a tiny dependency-free JSON reader (adapters) + writer-escape (JSONL).
  - `render.rs` — `render_human` (quiet-on-clean + positive sentence) + `render_jsonl`
    (versioned `dorc-lint-format/1` envelope carrying the coverage block).
  - `lib.rs` — `lint(files, oracles, options, runner, only) -> LintReport` + `list_sources()`.
  - `tests/adapters.rs` (9) + `tests/report.rs` (8) over a fake runner (anti-masking: raw bytes).
- **cli wiring** (`crates/cli/src/main.rs`) — `Invocation::Lint`, `parse_lint_args`,
  `lint_command`, and the REAL `SubprocessRunner` (`std::process`, stdin-piped, a `which`-style
  `tool_on_path` with Windows PATHEXT). Exit trichotomy: `EXIT_LINT_FINDINGS=1`,
  `EXIT_LINT_OPERATIONAL=3`; usage stays 2; the 10..19 dorc-semantic family is NOT reused (a
  ⊤-reject book is a FINDING). HELP documents the mode + the recommended CI line.
- **e2e** (`e2e/run.sh` + `e2e/lint-cases/`) — a strictly-additive `lint-cases/*/` loop after
  the untouched `cases/*/` loop; 3 cases (eval-wall findings, tools-absent info, jsonl
  envelope). Baseline 884 unit → 904 (+20); e2e 91 → 94 (+3).

## §2 — rung-oracle-solo reconnaissance (the non-droppable strain-note)

Reconnaissance ONLY per `27R` §8b (implement only what falls out with zero restructuring).

**What the pipeline does with an absent/empty book + oracles loaded** (measured: `dorc plan
--book=<empty.sh> -o <oracle>` with empty stdin): runs cleanly, `plan-summary sites=0`, emits
an empty apply artifact, exit 0. +SURE no crash. Crucially, the cli's `run()` performs ALL the
oracle-side lifts and lints (`dorc_oracle::lift`, `lift_predicts`, `verdict::VerdictSet::lift`,
`reserved::lint_oracle_reserved_names`, `marker::check_dialect_marker`,
`check_wrapper_peel_coherence`) BEFORE the book is parsed — they are book-independent by
construction.

**What breaks / what a real design must re-think:**
- The oracle-side validation logic is **not factored for reuse**: it lives inline in the cli's
  `run()` (main.rs), interleaved with book state and emitting straight to stderr via
  `report_at`, not as structured findings a lint source can consume. A clean rung-oracle-solo
  wants either a `dorc_oracle::validate(&[&str]) -> Carrier<()>` book-free entry, or the lint
  crate re-calling the individual PUBLIC book-free lints. ~SUSPECT the former is the right
  factoring (it is the `diag-api-design-for-keeps` posture applied to the oracle side).
- **Cell-level cross-oracle coherence** (kind-vocabulary conflicts, cross-oracle selector
  minting) genuinely needs a book: those surface only when a book exercises BOTH oracles'
  cells through `classify`. -GUESS a meaningful fraction of oracle-quality signal is
  intrinsically book-coupled and cannot move to rung-oracle-solo.
- **dual-peel coherence** (`check_wrapper_peel_coherence`) IS book-independent and would be a
  strong rung-oracle-solo lint, but it is a PRIVATE cli fn returning a bool + emitting via
  `report_at` — not reusable without factoring. Named `seam-oracle-validate-factoring`.

**What fell out with zero restructuring:** the `verdict-body` source already lints oracle files
(via `ctx.oracles`) with no book — the rung-file tier gives oracle authors hot-loop value now.
A fuller rung-oracle-solo source (surfacing reserved-name / marker / predict-out-of-dialect /
dual-peel diagnostics over oracles-with-no-book) is cheaply addable for the already-PUBLIC
lints, but blocked on factoring for dual-peel; **deferred by direction**, not built.

**§3 shared hint-lane finding (the human's open mechanism question):** the diag machinery
ALREADY supports the conductor's input-staged factoring. `source-analysis-diagnostics` literally
runs the no-world pipeline prefix (parse→cfg) and surfaces the SAME `Carrier` diagnostics that
`plan`/`apply` emit with more inputs — passes needing probe facts (a phased `classify` with a
world) simply are not run, not stubbed/faked. +SURE for parse/cfg. No new seam is needed for the
factoring itself in the sketch; ~SUSPECT a first-class "pass-input manifest" becomes worthwhile
only once more passes are lint-exposed (a bigger round's concern) — flagged, not built.

## §3 — Deviations from 27R (one paragraph each)

- **dev-source-selection-is-a-flag** — `27R` §5/§8 say "naming a source as an arg runs a
  subset" (brew's positional-checks model). Positionals here are FILES, so subset selection is
  a `--source NAME` (repeatable) flag instead. Same capability, unambiguous; brew can afford
  positional checks because it takes no file args.
- **dev-unmodeled-inventory-is-light** — `27R` §2 item-2 asks for "the count of downstream
  MODELED sites each wall degrades". The precise count needs the effect classification threaded
  with the loaded oracles (which sites are modeled). The landed source reports the wall count,
  the first-wall line, and an APPROXIMATE downstream-LEAF count (all downstream commands, framed
  as approximate), never over-claiming. Precise count = `seam-unmodeled-degradation-count`.
- **dev-verdict-body-pipeline-only** — `27R` §2 item-3 lists `!`-on-status, `|| true`
  flattening, AND terminal-pipeline tails. Only the terminal-pipeline is AST-detectable: `!` and
  `|| true` are OUT of the verdict dialect, so they never lift and instead surface through
  `source-analysis-diagnostics` as `predict-out-of-dialect` give-ups (still visible to the user).
  A dedicated precise lint = `seam-verdict-bang-and-ortrue-flattening`. Falsification-first,
  may-under-report posture honored (`rul-unprovable-rides-the-vouch`).
- **dev-e2e-findings-via-native** — `27R` §7 lists "findings present via inert stub
  shellcheck/checkbashisms on PATH". The e2e demonstrates findings-present via DORC-NATIVE
  findings (analysis + unmodeled), which is more robust; the stub-tool spawn path is covered at
  the unit tier over the fake runner. See `tc-lint-e2e-stub-tools-spawn` — a real Windows
  `dorc.exe` spawns via CreateProcess, which cannot run an extensionless POSIX shebang
  `shellcheck` stub, so a cross-platform stub-on-PATH e2e is genuinely fragile.
- **dev-no-color-in-crate** — `27R` §5 mentions tty-detected color / `NO_COLOR`. The renders are
  pure String producers (kept color out of the deterministic crate); the cli edge already
  carries `anstream`/`anstyle` and can colorize later. Not built this round.
- **dev-candidate-4-not-built** — `27R` §2's candidate 4 (`279f` §5 constant-rc-mark) was the
  stretch/priority-6 item; not implemented. Named as a seam.

## §4 — Named seams (do not build; carry)

Carried from `27R` §2 out-list + §8 (unchanged, still deferred): strip-floor lint (strip +
two-binary `dash -n`/`posh -n`, mind `fence-rejection-rc`) · the `24S:A6` wrapper-oracle bar ·
the `24T:P-A4` carrier bar · munge-lint/version-role reservations (`27Q` §2 MH2) · fix-modes ·
comment-directive suppression machinery · doctor-style environment checks · repo discovery
(zero-arg invocation) · LSP · diff-aware filtering (reviewdog) · SARIF/checkstyle emitters ·
lint result caching (kSTATE-adjacent) · shellcheck⇄checkbashisms double-report dedupe (v0
reports both, source-tagged) · fix-suggestion confidence tiers · `seam-lint-lock-manifest`
(`27R` §8b, kSTATE-adjacent).

New this round:
- **seam-unmodeled-degradation-count** — the precise "downstream MODELED sites each wall
  degrades" count; needs the effect classification threaded with oracles.
- **seam-verdict-bang-and-ortrue-flattening** — the `!`/`|| true` verdict-body lints; needs
  lexer-level bang/or-list support the verdict dialect does not expose.
- **seam-oracle-validate-factoring** — a book-free `dorc_oracle::validate` entry (or lint calls
  to the public book-free lints), to lift the oracle-side validation out of the cli's inline
  `run()` and enable a fuller rung-oracle-solo source (esp. dual-peel coherence).
- **seam-lint-color-render** — tty/`NO_COLOR` color at the cli edge (`anstream` already present).
- **seam-lint-real-tool-spawn-e2e** — an e2e that exercises the REAL SubprocessRunner against a
  cross-platform stub (see `tc-lint-e2e-stub-tools-spawn`).
- **rung-probe** (`27R` §8b) — probe-inclusive lint stays SEAM-ONLY: it is the plan pipeline's
  advisory surface wearing a lint hat; NEVER a second probe path. Reuse, don't rebuild.

## §5 — tc-* flags (conservative lean taken; the human re-rules cheaply)

- **tc-lint-operational-exit-code** — lint operational errors exit **3** (golangci-lint 3=Failure,
  shellcheck 3=bad-invoke). Distinct from clean(0)/findings(1)/usage(2), outside 10..19. The
  conservative lean; the human may prefer a distinct code per operational class (golangci's
  5/6/7 model) later.
- **tc-lint-fail-on-default** — `--fail-on` defaults to **error** (hot-loop mercy; the CI line
  tightens to `warn`). `27R` §6 tension-fail-on-default left this UNRESOLVED; implemented the
  error-only lean, flagged.
- **tc-lint-e2e-stub-tools-spawn** — the real-tool-spawn e2e is NOT done (Windows CreateProcess
  can't run a shebang `shellcheck` stub; a cross-platform stub needs both an extensionless script
  and a `.cmd`/`.exe`). Adapter parse logic is unit-tested over the fake runner instead. Judgment:
  keep at unit tier + seam the real-spawn e2e.
- **tc-lint-shellcheck-dialect** (§4 asked for a one-line rationale) — shellcheck runs with
  `-s sh`: dorc's dialect IS POSIX sh, so `-s sh` is the correct portability lens for BOTH
  stripped-marked oracle text and unmarked books, and it overlaps checkbashisms deliberately
  (both report portability). Latitude taken; re-rule if book-vs-oracle want different lenses.
- **tc-lint-source-selection-flag** — `--source NAME` vs brew's positional (see
  `dev-source-selection-is-a-flag`).

## §6 — Verification + notes for the conductor

- Four gates GREEN at tip: `cargo fmt --check` · `clippy --workspace --all-targets -D warnings`
  · `cargo deny check licenses bans sources` · `typos spike`. Full suite: 904 unit/integration/
  doc tests pass. `sh e2e/run.sh`: all 94 (91 round-trip + 3 lint) pass. Comment budget: **9**
  added inline `//` (≤25).
- A manual read-only smoke of the REAL binary confirmed: `--list-sources`, human render (exit
  1 on wall errors), `--format=jsonl` (envelope + coverage + findings), `--no-tools` (no
  tool-absent), `--expect-files` mismatch (exit 3), `--require-tools` absence (exit 3), and the
  clean positive sentence (exit 0). Real `shellcheck`/`checkbashisms` were NOT installed on the
  dev box, so the absent-path was exercised for real; the present-path is unit-tier only.
- Cosmetic: the e2e final line still reads "all 94 e2e ROUND-TRIPS passed" though 3 are lint
  cases (not round-trips). Left unedited to avoid churning a load-bearing summary string;
  `count-drifts` already governs. Flag if the wording should change.
- NEVER-vouch reminder: everything above is process-evidence (my own tests/gates), not proof of
  correctness. The adapters have never been run against real shellcheck/checkbashisms output;
  the tolerant-text tier and json1 shapes are modeled from `27Ra`'s manpage reads, not live runs.
