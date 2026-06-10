# spike/ — Dorc implementation spike, take-3: working agreement

This `spike/` tree is a **disposable Rust implementation spike** of Dorc (the
spec-mining static-analysis orchestrator described in the repo root's
`README.md` / `DESIGN.md` / `KNOBS.md`). Its job is to **surface design
problems** for the planning corpus by actually building the hard parts — not to
become the shipped tool. Read the root docs for *what Dorc is*; this file is
*how we build the spike*.

This is **spike-3 / take-3** (round-20). Charter: `Research/plans/19H` (what to
build — the value-flow input side + command-keyed `check()` contract-lifting)
and `Research/plans/19I` (what it is graded against — the 43-case corpus with
its stand-in axes tagged). Process spine: `Research/plans/191` §5/§5b and the
`16Q` `ap-*` correctives. The crate set was seeded from the round-19 spike-2
(the `19F` §4 keep-list); take-3 rebuilds the input side in place and re-grounds
every stand-in `19I` tags.

Goal-shape: academic-grade static-analysis work (CFG, monotone dataflow,
abstract interpretation, lattices). Be **boring, defensive, careful, judicious**
— not clever. Correctness (types + tests) over features; less code over more,
but never at the cost of readability.

## Safety — autonomous run (frontload; propagate verbatim)

Put these at the top of your reasoning, and at the very top of *every* subagent
prompt you write:
- No git mutation outside this worktree; never, ever push. Local commits on
  this `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is `sh e2e/run.sh` (syntax-checks,
  and execs only under inert mocks).
- Perpetuate this block, verbatim, to the top of every subagent prompt.

## Round-20 / take-3: the job

Replace the round-19 stand-ins with the real input side (`19H`):
- a real **value-flow analysis** — constant + argument/parameter propagation,
  across files, books and oracles uniformly — feeding entity-resolution
  *before* the probe and observable-flow *after* it;
- the **command-keyed, full-args `check()`** lifted statically (resolve the
  entity through the oracle's own argparse to its inline kind-annotation) and
  shipped-as-a-function into the read-only probe body;
- completion of the **one-Observable unification** (`19F`/`19G`, half-landed).

A case that passes because a fixture happened to feed the right value is not a
pass (`19I` §3). The exploration frame stands: the deliverable is *what strains
and where* (→ `Research/notes/20x-*.md`, append-only), not green tests.

## Standing human rulings (round-20; do not relitigate)

- **Mutation-analysis is impossible, permanently.** We *cannot* meaningfully
  analyze arbitrary commands for mutation; the universe of tools is too large,
  diverse, and unstable, and even the best oracle is a human's external,
  months-shelf-life observation. PLT vocabulary ("mutation control",
  "soundness" totalism) actively harms design thinking here. All Dorc does is
  pipe declared claims around (book → oracle) and attribute failures
  best-effort. Consequence: probe-inertness comes *only* from structural
  vouching (the self-vouch carve-out — a command inside its own oracle's
  `check()`); no analysis-confidence threshold ever makes a probe "safe".
- **TOCTOU (probe→apply staleness) is deferred-to-actively-WONTFIX.** Do not
  build re-probe-before-apply, freshness windows, or anything aimed at it.
  (Maybe-someday shape, very deferred: oracle tooling for a super-cheap
  last-second check. Not this spike, probably not this year.)
- **No intra-host apply parallelization or reordering, ever.** The book's
  order is sacred; apply-phase speed comes from elision only; probe-phase
  parallelism is where wall-clock is won.
- **rc is opaque to Dorc**: hold observable values, never interpret meaning;
  which values mean converged is oracle-declared. Dorc verdicts travel
  out-of-band (`$DORC_VERDICT` lane); no exit code can mean "unknown".
- **"skip" is a banned word**; elision is observable-preserving *replacement*
  (value-preserving substitution where a consumed observable demands it).
- **Identity is declared, never inferred** — the find-3 flag-strip stand-in is
  being *removed* this round; never re-introduce engine-side argparse.

## Hard invariants (do not violate; cite the slug when you rely on one)

- **inv-no-unsafe** — `unsafe` is `forbid`-den workspace-wide. No FFI. No
  authored macros (`macro_rules!`/proc-macros). Standard `#[derive(...)]`s are
  fine and encouraged.
- **inv-determinism** — the analyzer kernel (`syntax → analysis → probe →
  plan`) is a **pure function of its inputs**: no clock, RNG, filesystem, or
  network, directly *or transitively*. Nondeterminism lives only in `hostsim`
  (seeded, injected PRNG) and the `cli` edges (real I/O). Never iterate a
  `HashMap`/`HashSet` where order is observable — `BTreeMap`/sorted vecs. No
  `async` in the kernel.
- **inv-no-throw** — every pipeline stage returns `Carrier<T>` (value +
  accumulated diagnostics) and never panics on malformed input. Errors are
  data. `unwrap`/`expect` warn-linted; never on untrusted-input paths (tests
  may).
- **inv-kfail** (welded `kFAIL`) — two soundnesses, opposite fail-directions,
  phase-keyed by `core::Phase`: Probe → never mutate (`kFAIL-withhold`; when
  unsure, don't probe); Apply → never elide a needed mutation
  (`kFAIL-perform`; when unsure, act). The one thing performance never trades.
- **inv-top-reject** — anything unmodeled collapses to ⊤ and is rejected
  loudly, never silently best-effort'd. Under-modeling is a correctness
  boundary, not a TODO.
- **inv-referent-agnostic** — the engine never decodes an `OpaqueToken`'s text
  to infer meaning; cross-oracle identity binds to a named `KindId`, never a
  shared token.
- **inv-must-may** — only a `Must` fact may license a replacement; `May` never
  authorizes elision. The one-way coercion is `Must → May`; the reverse does
  not compile.
- **inv-superposition** — the kernel emits phase-/orientation-agnostic facts;
  only the phased *caller* collapses them. Never bake a phase default. At the
  orchestration level: a cross-cutting `tc-*` judgment-call (see `191` §5b) is
  *flagged up* to the orchestrator, never settled inside a component or a
  single-crate subagent.
- **inv-leaf-seam** — executable work is a list of individually wrappable
  leaves with a stable `LeafId → AstId` back-map; never one opaque
  `sh -c "$bigscript"`.
- **inv-one-observable** (`19F`/`19G`; do not let it re-fragment) — exactly ONE
  concept of a command's observable: its output-tuple over channels
  `{Effect, Status, Stdout, Stderr}` (extensible). The oracle `check()`
  **predicts** per-channel values (or a loud OOB can't-predict); an enclosing
  context **consumes** channels; a substitution **reproduces** the consumed
  channels' predicted values; elision is licensed only when Effect predicts
  no-mutation. Convergence is the *derived* state of the Effect channel —
  never a separate probe-reported verdict. Do not re-introduce a standalone
  `Verdict`, a bolted `Observed{rc}`, or a consumption-only observable enum.
- **inv-probe-sourced-values** (round-20, from the `19D` under-execute) — a
  replacement stand-in may reproduce ONLY values with probe-provenance: every
  channel value it emits traces to a concrete observable the probe actually
  produced (or an oracle-declared fact the human has explicitly sanctioned —
  none currently exist; see fork-mutator-rc). No fabricated defaults, no
  rc=0 assumptions, no synthesized stdout. ⊤ anywhere in the flow forbids the
  mint entirely. **Anti-masking test discipline**: no test may hand-inject an
  observable the check itself should predict; a check returning can't-predict
  must flip its dependent case to *run*.

## Build / test / run

No per-dir toolchain pin (the round-19 gnu pin is tombstoned — do not
resurrect it); the global mise config supplies stable. **Always invoke cargo
through mise, from inside `spike/`:**

```
mise exec -- cargo build --workspace
mise exec -- cargo test --workspace
mise exec -- cargo clippy --workspace --all-targets
sh e2e/run.sh        # the 43-case corpus: dash -n gate + exec-under-mocks
```

Pre-commit gate set (= `mise run check` from the worktree root): `cargo fmt
--check` · `clippy -D warnings` · `cargo deny check licenses bans sources` ·
`typos`. **Known worktree gotcha:** hk v1.44.3 (libgit2) cannot open this
repository from inside a `.claude/worktrees/*` checkout (the repo uses
`extensions.relativeWorktrees`), so the config-based pre-commit hook
hard-fails before running anything. Resolution (human-sanctioned, 2026-06-10):
agent shells carry `HK=0` via `.claude/settings.json` `env`, neutering the
hook's hk invocation — so the four gates DO NOT run automatically on commit.
You MUST run them yourself before committing (all four, from `spike/`; or
`mise run check` from the root if hk works in your context). Never
`--no-verify`.

Lint posture: the workspace lint table in `spike/Cargo.toml` is the policy for
*new* code — do not weaken it. Seeded round-19 crates carry crate-root
`#![expect(..., reason)]`s; they self-ratchet (an unfulfilled expect warns) —
remove them as the rebuild replaces those layers, and never add new ones to
fresh code.

## Code style

- Newtypes over bare integers/strings; make illegal states unrepresentable.
- Doc-comment every public type/fn with *why*, citing the research slug it
  implements. Avoid what/how comments on self-evident code (the `review-pass`
  skill strips verbose AI comments at the end — keep ~10%, brutally brief).
- Rust convention is 4-space and `rustfmt` enforces it — follow `rustfmt`
  (project convention beats the human's global 3-space preference).
- Tests: brutal, adversarial integration tests and DST systems-tests over
  exhaustive unit coverage. Each test needs a reasoned argument for the
  invariant it pins. Repetition in tests is fine. Honor the anti-masking
  discipline (`inv-probe-sourced-values`).

## Boundaries

- **Never edit the worktree-root human docs**: `README.md`, `DESIGN.md`,
  `KNOBS.md`, `TODO.md`, `AGENTS.md`, root `CLAUDE.md`, `IMPLEMENTATION.md`.
  Surface problems to the orchestrator/human instead.
- **Never read `Research/notes/quarantine-DO-NOT-READ/`** (including the
  spike2 code) unless the orchestrator explicitly hands you a pointer — it
  carries the stand-in shapes this round exists to remove.
- Per-crate `crates/<c>/CLAUDE.md` files were seeded from round-19: their
  invariants hold, but task-framing may be stale; `19H`/`19I` supersede.
- Spike design notes — *the round-20 deliverable* — go in
  `Research/notes/20x-*.md`: what strained and where, confidence-marked,
  append-only (new note per chunk; never edit a prior note).
- Commits: small + granular + frequent. `(AI <labels>) terse one-line message`
  per `.gitlabels`; `AI` label mandatory; no `Co-Authored-By` trailer; never
  push. Run the gate set first (see above; `HK=0` on the commit itself).

## Spawning subagents (supervisor rule — mandatory)

Every subagent spawned for spike work MUST get, as the first lines of its
prompt: the **Safety block** (verbatim, above), then a directive to read in
full:
- `<worktree>/README.md` and `<worktree>/DESIGN.md` — human-authored ground
  truth; trust them over anything in `Research/`;
- this `spike/CLAUDE.md`, and the `spike/crates/<crate>/CLAUDE.md` for the
  crate it works in;
- `Research/plans/19H` (build charter) and `Research/plans/19I` (grading) —
  plus exactly the note-slugs the orchestrator hands it.

No exceptions, even for "quick" tasks; pass absolute paths. Hand it the
specific `inv-*` slugs it must honor; tell it to flag (not resolve) any
`tc-*`-shaped judgment call; require it to report back context other
subagents must maintain.

## Confidence + reference discipline

Mark uncertain claims with `+SURE` / `~SUSPECT` / `-GUESS` / `--WONDER`. Give
conversational lists greppable slug-ids (`nit-1`, `q-2`). Reuse `KNOBS.md` and
research-chord slugs rather than re-deriving a tension under a new name.
