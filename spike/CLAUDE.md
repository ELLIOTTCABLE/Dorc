# spike/ — Dorc implementation spike: working agreement

This `spike/` tree is a **disposable Rust implementation spike** of Dorc (the
spec-mining static-analysis orchestrator described in the repo root's
`README.md` / `DESIGN.md` / `KNOBS.md`). Its job is to **surface design
problems** for the planning corpus by actually building the hard parts —
**not** to become the shipped tool. Read the root docs for *what Dorc is*; this
file is *how we build the spike*.

This is now **spike-2** (round-19): the same disposable-spike contract, re-aimed at
the keystone the round-16 spike skipped. Charter:
`Research/plans/190-spike2-keystone-charter.md` (read it first). This tree is the
round-16 code, forked here and verified green — extend it in place.

Goal-shape: this is academic-grade static-analysis work (CFG, IFDS dataflow,
abstract interpretation, lattices). Be **boring, defensive, careful, judicious**
— not clever. Correctness (types + tests) over features; less code over more,
but never at the cost of readability.

## Safety — autonomous run (frontload; propagate verbatim)

Put these at the top of your reasoning, and at the very top of *every* subagent
prompt you write:
- No git mutation outside this worktree. Local commits on the worktree branch are
  fine (autonomous-commit is on, `ai/*`); never push; never touch git elsewhere.
- Don't exhaust external rate-limits; space network fetches; respect robots.txt.
- No global system-state mutation (no system package installs / system config).
  Worktree-local mise installs + config (incl. `mise trust`) are fine.
- Perpetuate this block, verbatim, to the top of every subagent prompt.

## Round-19 / spike-2: the keystone, and the exploration frame

Charter: `Research/plans/190-spike2-keystone-charter.md`. The job is the structured,
recursive, kind-typed entity-algebra (and the analysis riding on it) that the
round-16 spike skipped — the fact-domain re-key that kills the "poison wall" (an
un-oracled neighbour poisoning all downstream ambient-ness, so nothing elides on a
real book). Per-crate guidance lives in each `spike/crates/<c>/CLAUDE.md`.

The frame is state-space exploration: the deliverable is *what strains and where*
(→ `Research/notes/19x-*.md`), not green tests. Pursue correctness in order to find
where pursuing it is hard. Mapping a wall by attacking it is valuable; abandoning a
direction at a wall to explore elsewhere is equally valid — take notes either way,
don't grind a rabbit-hole.

Three process rules the round-16 spike got wrong (charter `ap-*`):
- `ap-1`: build the keystone (the `core` fact-key re-key) before more type-machinery
  — the re-key invalidates anything built on the old flat key.
- `ap-2`: the acceptance harness must execute or `sh -n`-check the rendered artifact,
  never text-diff it (last spike shipped a non-runnable `then`-clause green).
- `ap-3`: when you run `/adversarial-crosscheck`, rotate its target across the
  harness, charter-adherence, and the three seams
  (`seam-prov`/`seam-interproc`/`seam-finite`) — not only core soundness.

Two locked latitudes (charter `ch-*`): the parser is a disposable test front-end —
massage inputs past it, don't chase arbitrary shell-input or build the `kTYANNOT`
off-ramp stripper; and the entity-algebra is the first thing allowed to *give*
(simplify its shape before abandoning the keystone), though finite-height
termination stays a floor.

## Per-component CLAUDE.md = invariants · knobs · tensions; cross-cutting tensions escalate

A per-crate `CLAUDE.md` foregrounds three things: the relevant **invariants** (`inv-*`, the always-rules),
the **knobs** it touches (the `KNOBS.md` tensions), and the **tensions** — the *opposites* of invariants,
judgment-calls with no "always." The dangerous tensions are cross-cutting soundness-orientation calls
(collapsing `May`/`Must`, minting an elision-voucher, judging a fact proof-level-vs-tainted, strong-vs-weak
update, which-phase, which-user, holds-under-an-unreliable-oracle — the `tc-*` set). A single-crate worker
lacks the phase/user/orientation context to settle these, so **flag them up to the orchestrator; never resolve
them in isolation** — this is `inv-superposition` at the orchestration level (emit the fact; let the
context-bearing caller collapse it). Purely-local tensions (render-fidelity, parser-massaging) stay with the
component. The `tc-*` set + the exclusion-check axes are enumerated in `Research/plans/190` (§5b).

## Build / test / run

Toolchain is pinned per-dir via `spike/mise.toml` (`[env]` selects the
self-contained `x86_64-pc-windows-gnu` toolchain because this box has no MSVC
linker). One-time after the round-19 fork relocated the configs (mise keys trust by
path): `mise trust mise.toml; mise trust spike/mise.toml`. Then **always invoke
cargo through mise, from inside `spike/`:**

```
mise exec -- cargo build
mise exec -- cargo test
mise exec -- cargo clippy --all-targets
```

Plain `cargo` (no `+toolchain`) is correct here — the mise `[env]` override
makes gnu the active toolchain, so there is no `+toolchain` footgun.

## Hard invariants (do not violate; cite the slug when you rely on one)

- **inv-no-unsafe** — `unsafe` is `forbid`-den workspace-wide; it cannot be
  re-enabled with `#[allow]`. No FFI. No authored macros (`macro_rules!` /
  proc-macros) — they hand reviewers and other agents footguns. `#[derive(...)]`
  and the standard derives are fine and encouraged.
- **inv-determinism** — the analyzer kernel (`syntax → analysis → probe → plan`)
  is a **pure function of its inputs**: no clock, RNG, filesystem, or network,
  directly *or transitively*. This is what lets the whole pipeline run inside
  deterministic-simulation (DST) tests without dependency-injection ceremony.
  The *only* places nondeterminism is allowed: `hostsim` (seeded PRNG, injected)
  and `cli` edges (real I/O). Never iterate a `HashMap`/`HashSet` to produce
  output — use `BTreeMap`/sorted vecs where order is observable. No `async` in
  the kernel (state-machine DST per the Polar-Signals model; async would
  reintroduce scheduler nondeterminism).
- **inv-no-throw** (`dn-7`) — every pipeline stage returns `Carrier<T>` (`value`
  + accumulated `Vec<Diagnostic>`) and **never panics on malformed input**.
  Errors are data. `unwrap`/`expect` are warn-linted; never use them on a path
  that untrusted input can reach (tests may).
- **inv-kfail** (`dn-6`, welded `kFAIL`) — two soundnesses, opposite
  fail-directions, **phase-keyed** by `core::Phase`:
  - `Phase::Probe` → never mutate (`kFAIL-withhold`): when unsure, *don't probe*.
  - `Phase::Apply` → never skip a needed mutation (`kFAIL-perform`): when unsure,
    *act*. An `Unknown` verdict folds to "act", never to "skip".
  A shortcut/optimisation is legal only if it fails the conservative way *for its
  phase*. This is the one thing performance may never trade.
- **inv-top-reject** — anything unmodeled collapses to `⊤` (a `Top`/`Unknown`
  element) and is **rejected loudly, never silently best-effort'd**. The parser
  models only the sh subset the analyzer currently exercises; everything else is
  ⊤-rejected. Under-modeling is a *correctness* boundary (elision-soundness), not
  a TODO. The `⊤`-trigger set (eval, dynamic command names, `. "$dyn"`, recursive
  `$((…))`, lvalue-taking builtins) is fixed — see the synthesis note.
- **inv-referent-agnostic** (`W4`) — the engine never decodes an `OpaqueToken`'s
  text to infer meaning. Compare tokens for intra-script co-reference; resolve
  for display/provenance; never branch on "is this `nginx`". Cross-oracle
  identity binds to a **named `KindId`**, never a shared token.
- **inv-must-may** (`must-may`) — only a `Grade::Must` fact (implied by idiomatic
  structure, or oracle-declared) may license a skip. `Grade::May`
  (mined/distributional) is a hint that bootstraps the oracle library and
  **never** authorizes elision.
- **inv-superposition** — the analyzer kernel emits phase-/orientation-agnostic
  lattice facts; only the phased *caller* collapses them, by arguing the phase
  (`Bias`/`PhasedVerdict<P>`) and orientation (`May`/`Must`). The engine must never
  fold `May`/`Must` or bake a phase default — a baked posture is a wrong-skip under
  the opposite phase's `kFAIL`. It generalizes `inv-must-may`/`inv-kfail` from the
  verdict to *every* phase-sensitive fact (DESIGN "Same analysis, different
  fail-safe posture").
  <!-- /* retrofit 2026-06-06 (16x postmortem pass): cfg.rs/plan.rs cite inv-superposition throughout, but this round-16 invariant list — authored before the slug was coined — omitted it; added so an agent hunting the registry finds what the code was already written against. Source: notes/quarantine-DO-NOT-READ/16K §3. */ -->
- **inv-leaf-seam** (`dn-3`) — executable work is a list of individually
  wrappable leaves each carrying a stable `LeafId → AstId` back-map; **never one
  opaque `sh -c "$bigscript"`**. The probe projection is a leaf-id-preserving
  rewrite.

## Code style

- Newtypes over bare integers/strings; **make illegal states unrepresentable**
  (enums for finite choices; no "stringly-typed" anything).
- Doc-comment every public type/fn with *why*, and cite the research-chord / `dn`
  / `wo` / `KNOBS` slug it implements, so the rationale survives. Avoid
  what/how comments on self-evident code (the `review-pass` skill will strip
  verbose AI comments at the end — keep ~10%, brutally brief).
- 3-space indentation is the human's global preference, **but** Rust convention
  is 4-space and `rustfmt` enforces it — follow `rustfmt` here (project
  convention beats personal preference). Run `mise exec -- cargo fmt`.
- Tests: prefer brutal, adversarial integration tests and DST systems-tests over
  exhaustive unit coverage. Do **not** chase 100% coverage. Each test should have
  a reasoned argument for the behaviour/invariant it pins. Repetition in tests is
  fine (no DRY ceremony in tests).

## Boundaries

- **Never edit the worktree-root human docs**: `README.md`, `DESIGN.md`,
  `KNOBS.md`, `TODO.md`, `AGENTS.md`, `CLAUDE.md`. They are human-authored and
  human-owned. If one looks wrong, surface it to the user; don't edit it.
- Spike design notes — *the round-19 deliverable* — go in `Research/notes/19x-*.md`:
  what strained and where, confidence-marked, not authoritative.
- Commits: small + granular + frequent (mistakes included). On this `ai/*`
  branch, autonomous-commit is on. Style: `(AI <core-label>) terse message`, one
  line, `AI` label mandatory, no `Co-Authored-By` trailer. Never `git push`.

## Spawning subagents (supervisor rule — mandatory)

Every subagent this spike spawns MUST get, as the first lines of its prompt: the
**Safety block** (verbatim, above), then a directive to read **in full**:

- `<worktree>/README.md` and `<worktree>/DESIGN.md` — human-authored ground truth;
  trust them over anything in `Research/` (unreviewed LLM planning-slop).
- this `spike/CLAUDE.md` and the `spike/crates/<crate>/CLAUDE.md` for the crate it
  works in — the invariants + its per-component task.
- `Research/plans/190-spike2-keystone-charter.md` — the spike-2 charter.

No exceptions, even for "quick" tasks; pass the absolute paths. Then hand the
subagent the specific invariant-slugs and research-chord slugs it must honor, and
tell it to report back any context that *other* subagents must maintain. This is
enforced by supervisor discipline (the prompt template), not a hook — do not let
it slip.

## Confidence + reference discipline

Mark uncertain claims in notes/reasoning with `+SURE` / `~SUSPECT` / `-GUESS` /
`--WONDER`. Give conversational lists greppable slug-ids (`nit-1`, `q-2`), not
bare numbers. Reuse `KNOBS.md` slugs and research-chord slugs rather than
re-deriving a tension by a new name.
