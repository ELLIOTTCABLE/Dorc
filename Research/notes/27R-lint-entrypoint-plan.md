# 27R — `dorc lint`: entry-point + CLI/UI sketch — conductor plan

AI-authored (Fable conductor, 2026-07-18, session worktree `r27-lint-conduct`, branch
`ai/r27-lint` off `ai/main` @ `49e2965`). Status: PLAN-OF-RECORD for a one-lane Opus
build; the human's prompt is the charter. Authority: root docs, `spike/CLAUDE.md`,
`plans/27C`, `notes/27Q` outrank this. The builder's landing note (expected `27S`) will
carry the as-built ledger; this note carries the plan and the conductor's design
directives.

## §0 — Charter (the human's ask, restated)

A general entry-point + CLI/UI for `dorc lint`: an oracle-author-focused (NOT
oracle-exclusive) grab-bag mechanism ala `brew doctor` / `mise doctor` — a home for a
variety of mechanical authorship-related reporting. Constraints, verbatim-in-spirit:

- eventually both interactive and CI-style activation; interactive CLI = unstable,
  readability-first; CI invocation = stable, machine-readable, same information.
- garner existing value from the universe: mechanize `checkbashisms`, `shellcheck`,
  and similar tools when present; stay agile around their upstream output changes
  (effectively `dorc strip` machinery + light, TOLERANT line-number remapping over
  "stuff that looks like line numbers" — never a brittle parser of their output).
- become a home for hint/warn tooling that ALSO ties into plan/apply runs, but
  exposed here WITHOUT a plan-in-construction/probe-results feeding the reasoning
  (mechanism deliberately unspecified by the human; explored in §3).
- pluggable, so future methods of "helping oracles become better" slot in.
- no sharp edges: must not break the authorship hot-loop or it won't get used.
- deeply respect sibling-engine invariants; never soften correctness behaviours.

Spike-tier scope: "mildly minimal" sketch, real code, seams named for what's deferred.

## §1 — Where it sits (architecture directives)

- **dir-lint-is-a-mode**: a new `lint` mode beside `probe|plan|apply|why|strip` in
  `crates/cli` — but per the thin-driver mandate (`crates/cli/CLAUDE.md`
  scope-boundary), the machinery lives OUT of `main.rs`: a new small workspace crate
  (working name `crates/lint`) hosting the source registry + pure passes.
- **dir-runner-is-the-di-seam**: external tool invocation (shellcheck, checkbashisms,
  someday `dash -n`/`posh -n`) is non-hermetic — it goes behind ONE injected trait
  (strawman `ExternalToolRunner`: bytes+args in, {tool-rc, stdout, stderr} out, plus
  availability probe). The real subprocess impl lives at the cli edge
  (io-at-edges-only); unit tests inject fakes; e2e uses `PATH`-shadowed inert stubs
  per harness law. The `lint` crate itself stays deterministic given a runner
  (inv-determinism posture: same shape as hostsim's seam).
- **dir-registry-stays-dumb**: pluggability = a thin `LintSource` trait + a static
  `Vec<Box<dyn LintSource>>`. No dynamic discovery, no config, no plugin loading;
  "pluggable" here means "adding source N+1 is a small, local diff", nothing more.
  (tension-registry-abstraction-cost, §5.)
- **dir-no-license-plane-contact**: lint findings are ADVISORY ONLY. Nothing in the
  lint lane may mint, widen, or influence a claim/license/fact; lint never touches
  the `core::claim` tiers. It is a pure reporting surface (kWARN-rich's home turf).
  Silence-licenses-nothing runs in both directions: lint-clean licenses nothing too.
- **dir-lint-never-probes**: lint contacts no hosts, ships no probes, reads nothing
  but the files given (plus spawning the sanctioned local read-only tools). It must
  be safe to run in a pre-commit hook on an airgapped box.

## §2 — The v0 source set

1. **source-analysis-diagnostics** — run the existing pure pipeline
   (parse → cfg → classify) over the given files with NO probe results and NO world;
   surface the Carrier diagnostics as findings. Nearly free; makes `dorc lint`
   immediately non-trivial. (The diag API is the design-for-keeps exception —
   diag-api-design-for-keeps — so building on it is the sanctioned direction.)
2. **source-unmodeled-inventory** — the static half of the plan-time hint machinery:
   per book, the unmodeled command families, the first-wall position, and the count
   of downstream modeled sites each wall degrades. Severity info/hint. This is the §3
   demonstrator: plan/apply-adjacent reasoning with zero probe input.
3. **source-verdict-body-status-flattening** — oracle-contract §3 mechanicals inside
   `__is_converged` (and sibling verdict-bearing) bodies: `!` applied to the
   answering status; `|| true` flattening; terminal-pipeline tails where the status
   answered isn't the tool-under-description's (falsification-first subset only —
   rul-unprovable-rides-the-vouch's posture: the check may under-report, never gate).
   Candidate 4th: the banked `279f` §5 item, verdict/observe mark on a constant-rc
   line — include if the existing analysis makes it cheap, else name it a seam.
4. **source-external-shellcheck** + **source-external-checkbashisms** — §4.

Out of scope v0, named as seams in the landing note: strip-floor lint (strip + two-
binary `dash -n`/`posh -n` — mind fence-rejection-rc: never depend on a rejected
construct's rc/error-text semantics, display-only), the `24S:A6` wrapper-oracle bar,
the `24T:P-A4` carrier bar, munge-lint/version-role reservations (`27Q` §2 MH2),
fix-modes, suppression machinery, doctor-style environment checks, repo discovery
(zero-arg invocation), LSP.

## §3 — The shared hint-lane (the human's open mechanism question)

Frame handed to the builder (conductor strawman; explore + strain-note, don't weld):
the engine's hints/warnings are ALREADY structured diagnostics accumulated through
Carrier; what plan/apply adds is more INPUTS (probe facts, a world), not a different
channel. So factor by input-requirement: every diagnostic-producing pass is
implicitly staged by what it consumes — (parse) ⊂ (parse+oracles) ⊂ (+CFG topology) ⊂
(+probe facts) ⊂ (+apply results). `dorc lint` = run the no-world prefix; every pass
whose inputs exist fires; passes needing probe facts simply never run (they are NOT
stubbed, faked, or fed defaults — inv-probe-sourced-values' spirit). Under this frame
the "ties into plan/apply" property is free: the same pass fires in `plan` with more
inputs and in `lint` with fewer, emitting through the same diag API, rendered by
whichever surface is active. Builder explores: does the current diag machinery
already support this factoring, or does it want a small seam (e.g. a pass-input
manifest)? Flag tc-* if the seam is cross-cutting; do not restructure the pipeline
for it in a sketch.

## §4 — External-tool mechanization (the agility doctrine)

- **dir-strip-then-lint**: marked files are stripped through the REAL strip
  machinery (parser-backed; strip-is-pure-erasure) before external tools see them —
  never a parallel regex-strip (the "simple gsub" misread is a known root-doc-queue
  fix item). Unmarked files pass through strip unchanged by construction, so the
  path is uniform. The strip API grows a LINE-MAP return (stripped-line →
  original-line; whole-deleted annotation-lines are the only shifts) — kernel-pure.
- **dir-tolerant-adapters**: per-tool adapters, thinnest possible, with a hard
  degradation ladder: (a) if the tool offers a documented machine format
  (shellcheck `-f json1`), prefer it; (b) on ANY parse failure, degrade to generic
  text handling — remap "things that look like line numbers" (`:NN:`, `line NN`)
  through the line-map, pass message text through verbatim; (c) on total confusion,
  emit the tool's raw output as one opaque finding block. Upstream format drift may
  cost precision, NEVER a crash and NEVER silence. Findings carry a remap-fidelity
  tag (exact | approximate | none).
- **dir-absent-is-info**: a tool not on PATH = one info-severity finding per run
  (not per file), never exit-affecting by default; a strawman `--require-tools` (or
  similar) hard-fails for CI installs that want the guarantee. A tool that exits
  nonzero WITH parseable findings is just findings; nonzero with none is a single
  warn-severity operational finding. Never crash the lint run for a tool's tantrum.
- **dir-paths-stay-yours**: findings always name the user's original path and
  original line numbers; temp-file paths and stripped-line numbers never leak into
  any surface.
- Dialect selection for external tools (e.g. shellcheck `-s sh` for stripped marked
  text vs shebang-respect for unmarked books): builder's latitude, one-line rationale
  in the landing note.

## §5 — CLI/UI + output contract

- Strawman surface: `dorc lint <files...> [-o …|--oracle-dir …] [--format=human|jsonl]
  [--fail-on=error|warn|never] [--no-tools]` — spellings are builder latitude
  EXCEPT: no config file, ever (kOOB redline: no YAML/frontmatter/pragma; flags
  only), and no comment-directive suppression for dorc-native findings
  (marker-gates-syntax-only: the dialect marker stays the closed set of one
  comment-parse; shellcheck's own `# shellcheck disable=` directives pass through
  untouched — they're shellcheck's business, in shellcheck's input).
- **dir-two-renders-one-model**: one finding model, two renderers. Human render:
  unstable-by-declaration, grouped per file, source-tagged, severity-tiered,
  quiet-on-clean (one summary line), tty-detected color with `NO_COLOR` respected.
  Machine render: JSONL, versioned envelope line (strawman `dorc-lint-format/1`,
  mirroring `dorc-records/1` framing taste), one finding per line:
  {path, line, col?, severity, source, code, message, remap}.
- **dir-stability-split** (the stability-ledger applied to lint): the ENVELOPE/field
  schema is versioned and stable; dorc-native finding CODES are slugged, namespaced,
  append-only (a code is never re-read to mean something else — same posture as
  `__role` names); the finding SET is explicitly unstable-and-improving (verdicts-
  never-stable; plan-as-API is the named failure-mode — nothing may gate on "the
  same findings as last release", only on severity thresholds).
- Exit codes: 0 = ran, nothing at/above `--fail-on`; 1 = findings at/above
  threshold; 2 = usage (matches existing). Default `--fail-on` leans `error`
  (hot-loop mercy; CI tightens to `warn`) — tc-flagged, §6. Lint does NOT reuse the
  10..19 dorc-semantic fast-fail family: a book with a ⊤-reject construct is a
  FINDING here (that's the product), not an exit-10.
- **dir-deterministic-output**: findings sorted (path, line, source, code); external
  tool nondeterminism normalized at the adapter; identical inputs + identical fake
  runner ⇒ byte-identical output. e2e-able by construction.

## §6 — Open tensions (conductor → human; numbered for reply)

1. **tension-adapter-strictness-vs-churn** — json1-precise vs tolerant-text-remap
   (validation vs long-term maintainability). Directive taken: both, as a
   degradation ladder (§4). Residual: the generic remap can misattribute a line
   when upstream output drifts; priced as remap:approximate, interactive-only harm.
2. **tension-registry-abstraction-cost** — a trait + Vec today vs a richer plugin
   story later (simplicity vs future home). Directive taken: dumb registry;
   the trait boundary is the reserve, nothing else.
3. **tension-ci-stability-vs-verdict-churn** — CI wants "same output forever";
   doctrine forbids promising finding-set stability. Directive taken: §5
   dir-stability-split. Residual for the human: is code-stability (append-only
   namespace) a promise we want to make this early?
4. **tension-fail-on-default** — exit-1-on-warn (linter convention) vs
   error-only-by-default (hot-loop mercy). Lean: error-only default. UNRESOLVED —
   builder implements the lean, tc-flags it, human re-rules cheaply later.

## §7 — Build topology + process

- Conductor worktree: `.claude/worktrees/r27-lint-conduct`, branch `ai/r27-lint`
  (this note rides it). Builder: Opus-class, OWN isolated worktree, branch
  `ai/r27-lint-build` step-zero'd onto `ai/r27-lint`'s tip (stale-main hazard per
  standing law), step-0.5 `mise trust`, step-one root-doc reads. Builder brief
  carries: the spike safety block verbatim · the invariant pin-list (inv-determinism
  · inv-no-throw · io-at-edges-only · thin-driver · kOOB redline ·
  stability-ledger · strip-is-pure-erasure · fence-rejection-rc · skip-banned in
  code/design text · empty-world posture) · naming discipline (`270` §1) · comment
  budget (inline `//` only, hard cap + counting command) · no-BLESS-ever · the
  sonnet sub-spawn clamp (builder does the work itself) · four gates + foreground
  e2e before every commit-chunk · granular `(AI …)` commits.
- Builder deliverables: working `dorc lint` per §§1–5 · unit tests (line-map,
  adapters over a fake runner, jsonl shape) · e2e cases (stubs-on-PATH: findings
  present; tools absent; `--format=jsonl`) · landing note `27S` (as-built ledger,
  strains, tc-* flags, seams incl. §2's out-list) · zero edits outside
  `spike/` + `Research/notes/27S*`.
- Prior-art sweep (doctor-UX, shellcheck/checkbashisms interfaces, aggregator
  linters, machine formats, transformed-source lint remapping precedents) is
  running as a parallel gatherer; its digest lands in the scratchpad and the
  stealable-patterns shortlist gets folded into the builder brief + appended here
  before dispatch.

## §8 — Fold instructions (human)

Everything is on `ai/*` branches; nothing pushed. Expected end-state: `ai/r27-lint`
(plan note + any conductor addenda) and `ai/r27-lint-build` (build atop the plan
commit). Suggested fold: review `ai/r27-lint-build`, then fold it (it contains
`ai/r27-lint`'s commits as ancestors); LIVING_STATUS pointer line rides the
conductor's close-out commit.
