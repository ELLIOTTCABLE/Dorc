# 30Nf — the spec-promotion lane: `30I:step-8-promote-executable-specification`

> Tier: **LLM-authored, builder (Opus-class)**, lane `ai/r30-spec-promotion` from
> `ai/r30-conduct@3cb12e02`. Seven code commits, `076bc13c..8c6e829b`, plus this report.
>
> Read with: `plans/30I` (THE spec; the work order's step 8 and §13's specimen matrix are this
> lane's) · `notes/30Ne` §13 (the handoff this executes) · `notes/30Ib` §15/§16 (the loader and the
> acquisition edge this rests on) · `notes/30N` §3/§4 (the rulings that bind, and the human queue
> this feeds).
>
> HEADLINE: **all three `load30-*` XFAILs are PROMOTED**, and getting there cost two engine repairs
> and one harness capability, because the pins were red for reasons nobody had measured. The one a
> fold reviewer should read first is `fnd-multipart-never-placed-anything-in-production` (§2): the
> multipart artifact form has never, in any real invocation, published a dependency — including in
> the case minted to demonstrate it.

## §1 — The promotion inventory

| pin | disposition | what its target asserts | golden movement |
|---|---|---|---|
| `load30-rooted-shared-dependency` | **PROMOTED** | root value-flow reaches two entrypoints; the shared dependency's helper is live at both; the diamond publishes ONCE | placeholder → real transcript; `expected.ran` UNMOVED |
| `load30-two-point-frames` | **PROMOTED** | one entrypoint at two source points; the regional `sm_pick` wins inside the subshell; the guard's REUSE arm holds | placeholder → real transcript; `expected.ran` UNMOVED |
| `load30-subshell-errexit-fallback` | **PROMOTED** | regional `unset -f`; the `\|\|` fallback source under `set -e`; the subshell's definitions die at `)` | placeholder → real transcript; `expected.ran` UNMOVED |
| `p-x-sentinel-value-conjunct` | **RINGFENCED, untouched** | — | — |
| `p-x-loop-population-closes-over-literal-members` | **RINGFENCED, untouched** | — | — |

**`expected.ran` DID NOT MOVE IN ANY OF THE THREE**, and that is the promotion's own evidence. A
scoped bless rewrites `expected.ran` from what actually ran; it wrote back the committed bytes
unchanged in all three cases. The target run sets those pins were authored against are exactly what
the real path now produces — the promotion is the behaviour arriving, not the assertion relaxing.

**Golden movement, classified.** Uniform across the three, and the smallest movement the promotion
could have: the one-line placeholder (`XFAIL target transcript is minted only after …`) is replaced
by the case's real probe-then-apply transcript. In every case the apply block is the book
BYTE-FOR-BYTE — none of the three commits probe records, so nothing converges, nothing elides, and
the artifact is the authored bytes. The probe block is the site-unresolvable comment list (a book
`.` is unmodeled and walls — `opaque-poison-is-the-product`, unchanged by this lane). No other
golden in the corpus moved; `bless:dry` writes nothing and leaves a clean tree (§7).

**`floor30-dot-loader-function-errexit` is byte-identical** (`git diff 3cb12e02..HEAD` over it is
empty), as is `floor30-inline-dot-boundary`. `bless:floor` was never run.

**`head-expected.ran` is discharged** on all three, and only on those three. The brief keeps it
"until promotion proves behaviour did not drift by another route": the proof is above — the run set
the pin was authored against is the run set the promoted case asserts, byte-for-byte, and the file
is consulted only under the XFAIL lens, which is gone. It survives everywhere else it exists.

## §2 — `fnd-multipart-never-placed-anything-in-production` (the finding, and the repair)

The three pins were failing at one gate — `ap-2-exec`, with `dash: .: cannot open ./alpha.dorc.sh`.
That is not a load-model failure. It is the artifact never carrying what the book sources.

**The measurement.** `dorc plan --artifact-dir out` over a book that sources a contracted package
published `plan.sh` ALONE and emitted `artifact-form-fallback`. Reproduced first on
`load30-rooted-shared-dependency`, then on a hand-built replica of
`emit30-multipart-publishes-its-dependency`'s own shape — the case minted to demonstrate the
multipart form. **That case has never taken the multipart form under the real binary.** It passes
because it asserts stdout alone, and `plan.sh` is byte-identical under all three forms
(`30Ne` §1), so the form it actually took was invisible to every gate.

**The cause.** `artifact::dependency_files` asked `placeable(authored)` of the path stored in
`snapshot.source_paths()`. For a book-sourced dependency that path is the CANONICAL key the loader
resolved (`main.rs::read_book_sourced` pushes `wanted` verbatim), and `invocation_cwd()` answers an
ABSOLUTE directory whenever the platform can say where the run stands. `placeable` refuses an
absolute path — correctly, as a path SHAPE rule. So every real invocation answered "unplaceable"
while every in-process test said the opposite, because a test's modelled cwd is the flat virtual one
(`Cwd::default()`) under which authored paths stay relative. Two worlds, one rule, no test spanning
them.

**The repair** (`076bc13c`). Mirroring is now stated against the LOAD CWD rather than a path's own
spelling: `Cwd::relativize` is the inverse of `resolve_operand`, sited beside it in `core::loadpath`
for the reason that module's header already gives, and `artifact::mirrored` composes it with the
unchanged `placeable` shape rule. A dependency INSIDE the load working directory mirrors to the
spelling the author used; one outside it is unplaceable, which is
`need-controller-paths-never-cross-hosts` holding rather than being lost. Measured after: the
replica publishes `wombat.dorc.sh` beside its plan, and `load30-rooted-shared-dependency` publishes
`alpha`, `beta` AND the transitively-reached `common`.

**The cell that was missing, now pinned** (`a_dependency_under_an_absolute_load_cwd_still_mirrors`):
the production cwd shape, which no test here had. Its sibling
(`a_dependency_outside_the_load_cwd_is_unplaceable`) keeps the refusal half honest.

## §3 — `fnd-unset-f-havocs-every-variable` (the second repair)

With the tree published, two pins greened and `load30-subshell-errexit-fallback` did not: its
`fallback.dorc.sh` was still absent, and the engine reported the `.` operand as
"a runtime-dynamic variable value (unset or branch-conflicted)" — for `SM_ORACLE_ROOT`, which the
book assigns unconditionally two lines above.

**Isolated by bisection over four synthetic books** (`||` alone, subshell alone, both, plus
`unset -f`): neither the subshell nor the `||` costs the resolution. `unset -f sm_pick` does.
`value::transfer_lvalue_builtin` read any leading `-` operand to `unset` as "which variable died is
unknowable" and havoc'd EVERY tracked binding. But `-f` says the operands name FUNCTIONS: the
variable plane is untouched, and both floor binaries agree. The over-approximation cost the `.`
operand its resolution, and a `.` whose operand is ⊤ resolves to no file — so the package was never
acquired, never bundled, and never published.

**The repair** (`446734f6`) models exactly one form and nothing more: a LEADING `-f`, because both
floor shells stop option parsing at the first non-option word, so `unset root -f` really does name a
variable and keeps the conservative reading. `-v`, combined flags, and dynamic operands are all
unchanged; a dynamic lvalue is refused a tier up at syntax
(`syntax-unsupported-unset-dynamic-lvalue`) and this does not reach that.

**FLAGGED UP — `tc-unset-f-precision-is-licensure-relevant`.** This is a PRECISION WIDENING in the
licensing direction: more variables resolve ⇒ more loads resolve ⇒ more definitions bind ⇒ more
licenses become possible. It is EXACT rather than heuristic (`unset -f` provably touches no
variable, and `rul-unsure-falls-toward-sh-parity` binds name resolution by name), and `30I` specimen
2 cannot exist without it — its regional `unset -f` sits directly above the load. But it is the
same SPECIES as the winner-shifting changes `28Q` §1 routes to license review, so it is flagged
rather than settled. Corpus effect measured: zero goldens moved.

## §4 — The harness capability, and why it is not a weakened question

`30Ne:tc-multipart-has-no-published-tree-assertion` was banked as "a harness question, not this
lane's". It is this lane's, because without it the three pins are red FOREVER for a harness reason:
`exec_check` ran the rendered plan text in an EMPTY throwaway sandbox, which is the flattened form's
world, and a book with a top-level `.` cannot run there whatever the engine does. Leaving them red
under a fresh "why" would have been a false signal about the engine.

**As built** (`1af33489`): a case declares `ARTIFACT_SET` (a bare presence marker). The runner then
gives the ROUND-TRIP DRIVE ALONE an `--artifact-dir` under its own scratch, requires exactly one
published generation, asserts the published `plan.sh` equals the apply block on stdout, and runs the
exec gates FROM that generation — `cd <artifact> && sh ./plan.sh`, which is the cwd `30I` §7.6
gives a multipart artifact. `capture_run` gained the same optional root, so gate-5 (argv-echo) and
gate-6 (dual-rail) stop being vacuous for these cases too.

**THE OPTION NOT TAKEN, and why it matters.** The obvious cheaper fix is to copy the case's own
authored sources into the sandbox — `capture_floor_stdout` already does exactly that for gate-9. It
is refused, in the code comment and here: it would green a case against CONTROLLER-side files the
target never receives, proving nothing about what the artifact ships. That is the shape the brief
calls a re-horizon dressed up. Running the artifact's own published tree is the opposite — it is a
STRICTER question than the corpus asked before.

Falsifiability: `artifact_set_selftest` drives `published_generation` at zero, one and two
generations, on the corpus's own self-test precedent. Without the refusal, a case whose run
published nothing would measure the plan alone under a multipart name and pass.

**Discharged in passing**: `28K` §8 `res-book-ships-its-load-closure` — "named there, unbuilt", and
quoted in `pin28-variable-resolved-source-loads`'s own header as the reason no executing corpus case
had ever carried a top-level `.`. Three now do.

## §5 — The lowering: what was lowered, and the case for what was not

**A property→home table for the `30I` arc's own pre-build xfail suite** (the population §13 stages
for lowering — the three `load30-*`, the two `aggregate30-*` already promoted by `30La`):

| property | where it is pinned | e2e retained? |
|---|---|---|
| `.` resolution: absolute / relative / slash-less / unknown cwd | `core::loadpath` (native) | no e2e owns it |
| the INVERSE — a resolved path relative to the load cwd | `core::loadpath::relativize` (native, NEW) | no |
| mirroring at the production (absolute) cwd | `cli::artifact` (native, NEW) | no |
| a dependency outside the load cwd is unplaceable | `cli::artifact` (native, NEW) | no |
| a diamond dependency is placed ONCE | `cli::artifact` (native, NEW) | exercised end-to-end by `load30-rooted-shared-dependency`, but the RULE is native |
| `unset -f` touches no variable | `analysis::value` (native, NEW) | no |
| the guard's reuse/source arm, per condition | `analysis::funcenv` TABLE 5/6/8 (native, `30Ib` §6) | no |
| the three load-account projections | `analysis::funcenv` TABLE 8 (native, `30Ib` §6) | no |
| speaker minting vs. its two counterfactuals | `load30-speaker-minting-is-observable.loom` | YES — `30I` §13 specimen 6 designates ONE multipurpose e2e |
| root value-flow + shared diamond, EXECUTED | `load30-rooted-shared-dependency` | YES — compound |
| frame-scoped guarded loading at two source points, EXECUTED | `load30-two-point-frames` | YES — compound |
| subshell + errexit + `unset -f` scope, EXECUTED | `load30-subshell-errexit-fallback` | YES — compound |
| the multipart form reaching the real binary | `emit30-multipart-publishes-its-dependency.loom` | YES — the artifact-interaction compound |
| the two floor questions | `floor30-dot-loader-function-errexit`, `floor30-inline-dot-boundary` | YES — MEASURED ground truth; unlowerable by construction |

**NO E2E WAS DELETED, and that is a deviation I am leaving OPEN rather than self-endorsing**
(`dev-no-e2e-deleted`). The argument, so the conductor can overrule it cheaply:

1. The lowering instruction in `30I` §13 governs the arc's own pre-build xfail suite. Every member
   of that suite is a whole-product observation whose assertion is a RUN SET produced by a real
   shell — a thing no in-process test can make. Each of the three observes a DIFFERENT irreducible
   interaction (the table above), so deleting any one loses a cell.
2. What WAS lowerable in this arc was already lowered by builders 1–4; `30Ib` §6 is that inventory,
   and it is large. This lane added five more native pins at their ownership seats (table above),
   which is the same act in the same direction.
3. The two genuine over-coverage candidates I found both sit OUTSIDE this arc, and absorbing them is
   what `30I`'s "Neighboring work that stays out" forbids. They are named in §6 for whoever owns
   them.

**One honest weakness in a promoted case, recorded rather than papered over.**
`load30-rooted-shared-dependency`'s run set does NOT discriminate its include guard: if the guard
failed and `common.dorc.sh` were sourced twice, re-defining the same function, the run set would be
identical. What it does discriminate is root value-flow, both entrypoints loading, the shared
helper being live, and (now) the diamond publishing once. The guard's reuse arm IS discriminated —
by `load30-two-point-frames`, where a re-source would rebind `sm_pick` and change the logged argv.
The two cases are complementary rather than redundant, which is the reason both are retained.

## §6 — Over-coverage found, owned elsewhere, NOT acted on

- **`emit30-cross-custody-plural-helper-suspends` ⊂ `emit30-ambient-dependency-narrates`.** The
  former pins: a voucher whose helper binds from a third author's co-loaded file suspends, ships no
  record, and the apply is the book. The latter's ALPHA arm is that exact shape, its BETA arm is the
  unaligned variant, and it declares the same two diagnostics — while also observing through a run
  set, which the former has no mocks to do. The former's own header concedes its plurality is
  "incidental colour". It is a `28R` case; deleting it is an adjacent-arc act.
- **`sourcing-degrades-safely` (19I/20B).** Its property — a slash-less `.` resolves nowhere, walls,
  and the downstream command still RUNS — is natively pinned twice (`core::loadpath`,
  `analysis::funcenv`) and e2e-pinned in `pin28-variable-resolved-source-loads`. What it uniquely
  still carries is an EXECUTED proof that the poison wall degrades to run rather than to silence
  (it has mocks and a run set; `pin28-*` deliberately has neither). Retaining it is defensible on
  that alone. NOTE for whoever revisits it: the reason `pin28-variable-resolved-source-loads` has no
  mocks — quoted in its own header — is the harness limitation this lane removed, so that case could
  now become executable and subsume this one. That is a golden-moving change in another arc's case,
  and is not mine.

## §7 — Verification

- **`mise run both gate:full-quiet` — BOTH LEGS GREEN, rc=0, FOREGROUND**, at the final tip,
  Windows leg first (`preflight-bounds-before-spend`). WSL trust taken for the worktree and the
  nested `spike/verify/aeneas` config first (`wsl-trust-per-worktree`).
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`; `git status --porcelain` empty
  afterwards, so no golden would be rewritten.
- `mise run xfail:census` — **9 live pins, 1 reserved; NO horizon expired.** Both ringfenced pins
  present and unchanged. The census is unchanged by this lane: the `load30-*` XFAILs are e2e-lens
  pins (an `XFAIL` marker file), not `internal_tooling::xfail` registry rows, so promoting them
  moves no census entry.
- `mise run test:e2e` — full corpus green. Before the goldens were blessed, the ONLY failures in the
  whole corpus were the three placeholder content diffs; every structural gate passed. That is the
  measurement that says the promotion is real.
- `mise run clippy` clean under `-D warnings`; `mise run check-quiet` rc 0.
- **Comment budget: 24** added inline `//` lines —
  `git diff 3cb12e02..HEAD -- "*.rs" | grep -cE "^\+\s*//($|[^/])"` = 34, minus 10 `//!` module-doc
  lines (the briefed regex counts `//!`). Within the briefed ≤25. Rationale that would have gone
  inline lives in `///` docs on the items it explains.
- **Golden movement: the three promoted transcripts and nothing else** (§1).

## §8 — Deviations, each OPEN, none self-endorsed

1. **`dev-mirroring-repair-was-not-in-the-remit`** — the brief scoped this lane to promotion and
   lowering. Fixing the artifact form was not named, and it is step-7 work. Taken because the three
   pins are unpromotable without it and because leaving it would have left the multipart form
   demonstrably non-functional behind a green case. If the conductor would rather the repair rode a
   step-7 revision, it is `076bc13c` alone and lifts cleanly.
2. **`dev-value-plane-precision-widened`** — see `tc-unset-f-precision-is-licensure-relevant` (§3).
   Licensure-relevant by species, exact by argument, corpus-neutral by measurement.
3. **`dev-harness-gained-a-published-tree-lane`** — a new `ARTIFACT_SET` marker and an optional run
   root threaded through four gates. It answers a question `30Ne` banked for someone else. Opt-in,
   so no existing case changes cell; the SingleStream cell every corpus case sits in is untouched.
4. **`dev-no-e2e-deleted`** — §5. The named deliverable produced native pins and an argument rather
   than deletions.
5. **`dev-artifact-set-is-dir-form-only`** — the marker is a presence file, so a `.loom` case cannot
   declare it without a new frontmatter key (a closed vocabulary, `dorc_loom::FRONTMATTER_KEYS`).
   The consequence is concrete and unclosed: `emit30-multipart-publishes-its-dependency` now really
   does take the multipart form and publish its dependency, and NO GATE ASSERTS THAT — it is the
   same silence that hid the defect in §2, merely with the defect fixed. Closing it is one
   frontmatter key plus its refusal row.
6. **`dev-fmt-stages-what-it-rewrites`** — `mise run fmt` leaves the files it rewrites STAGED, so a
   later pathspec-scoped `git add … && git commit` silently carried an unrelated staged file into
   one commit (`f7c25231`, whose message was amended to name both halves). Nothing was lost, and
   the lane's commits are otherwise one slice each. Recorded because "commit by explicit pathspec"
   does not protect against a pre-staged tree, and the next lane should `git commit -- <pathspec>`
   or reset first.

## §9 — `tc-*` judgment calls, flagged UP

- **`tc-unset-f-precision-is-licensure-relevant`** (§3) — the sharpest item here. A value-plane
  precision widening that shifts what can be licensed, taken because a typed specimen requires it.
- **`tc-mirroring-refuses-outside-the-load-cwd`** — a dependency at `/opt/shared/pkg.oracle.sh`
  under a load cwd of `/ops/case` is now UNPLACEABLE, so a book sourcing a system-wide oracle
  directory falls back to the preserved tree and says so. That is the conservative reading of
  `need-controller-paths-never-cross-hosts`, and it is also the only reading that does not invent a
  destination. The alternative (a generated artifact-root variable, `30I` §7.4's third bullet) is
  explicitly staged later; this lane did not open it.
- **`tc-exec-gate-runs-the-published-plan-not-the-stdout-text`** — under `ARTIFACT_SET` the gate
  executes `<generation>/plan.sh` as a script argument rather than piping the stdout text, so `$0`
  differs between the two lanes. No corpus book reads `$0`; the equality of the two byte-streams is
  asserted separately. Flagged because it is a real difference in what gets executed.
- **`tc-load30-rooted-does-not-observe-its-guard`** (§5) — recorded so nobody later reads that case
  as pinning the include guard.

## §10 — Proposed steering text (conductor's to place; NOT edited by this lane)

**`spike/crates/cli/CLAUDE.md`, appending to `artifact-forms-derive-from-one-structure`:**

> Mirroring is stated against the LOAD CWD (`dorc_core::loadpath::Cwd::relativize`, the inverse of
> `resolve_operand`), never against a stored path's own spelling. Every source a book `.` reaches is
> filed under its CANONICAL key, which is ABSOLUTE whenever the edge could answer where the run
> stands — so a seat asking whether the stored spelling looked relative answers "unplaceable" for
> every real invocation while every in-process test, whose modelled cwd is the flat virtual one,
> says the opposite. That divergence shipped once and was invisible: `plan.sh` is byte-identical
> under all three forms, so a case asserting stdout alone cannot see which form it took
> (`30Nf:fnd-multipart-never-placed-anything-in-production`). A dependency OUTSIDE the load cwd is
> unplaceable rather than fudged, which is `need-controller-paths-never-cross-hosts` holding.

**`spike/crates/analysis/CLAUDE.md`, new bullet under **Law — the dangers**:**

> - **lvalue-builtin-flags-are-spelled-not-guessed** (`30Nf` §3) — `value::transfer_lvalue_builtin`
>   havocs EVERY tracked binding when it cannot say which variable an lvalue builtin wrote, and that
>   floor stands. The one modelled exception is a LEADING `unset -f`, whose operands name FUNCTIONS
>   by the builtin's specification, so the variable plane is untouched; only leading, because both
>   floor shells stop option parsing at the first non-option word and `unset x -f` really does name
>   a variable. Widening the exception set is licensure-relevant in the same species as a funcenv
>   precision change (`28Q` §1): a resolved variable resolves a load, a resolved load binds
>   definitions, and bound definitions license. The measured cost of the un-narrowed form was a
>   whole package never acquired (`30I` specimen 2).

**`spike/crates/cli/CLAUDE.md`, appending to the acceptance-harness section:**

> - **an-artifact-set-runs-from-its-own-generation** (`30Nf` §4) — a case declaring `ARTIFACT_SET`
>   gives its round-trip drive an `--artifact-dir`, and the exec gates run the PUBLISHED
>   `<generation>/plan.sh` from inside that generation — the cwd `30I` §7.6 gives a multipart
>   artifact. Exactly one generation is required; none means the run took a form that materializes
>   nothing, and every exec gate below would then have measured the plan alone in an empty sandbox
>   and passed. The published plan is asserted byte-equal to the apply block on stdout. Copying a
>   case's own AUTHORED sources into the sandbox is the refused alternative: it would green a case
>   against controller-side files the target never receives.

**`Research/plans/30I`, "Where the build stands"** — re-cut the owed line to:

> Not built, and owed: the textual-inlining LOWERING itself (its floor measurement
> `floor30-inline-dot-boundary` is minted and committed; the flattened form still refuses rather
> than inlines a book-sited load, deliberately and unscheduled). `step-8-promote-executable-
> specification` is LANDED (`notes/30Nf`): all three `load30-*` XFAILs are promoted against their
> unmoved target run sets, the multipart form's placement defect is repaired, and the e2e harness
> executes a declared artifact SET from its own published generation.

and in the work order, mark `step-8-promote-executable-specification` complete.

## §11 — The `30I` arc-close residue, after this lane

1. **The textual-inlining LOWERING** — unblocked by the minted floor cell, deliberately unscheduled
   (`30N` §3 `adj-endorse-flattening-refusal-posture`). A value-add for the human to sequence.
2. **`dev-artifact-set-is-dir-form-only`** (§8 item 5) — the loom form cannot declare an artifact
   set, so the compound artifact case still asserts stdout alone. One frontmatter key.
3. **The four `[unwritten:]` registers from `30Ne`** plus `--artifact-dir`/`--form` undocumented on
   the help page — the prose queue (`30N` §4 item 9). Unchanged by this lane; no new code was
   minted here.
4. **`p-x-sentinel-value-conjunct`** — still red, still waiting on the human's
   `rule-sentinel-value-conjunct` ruling. Its trigger text remains accurate.
5. **§14's open pins are untouched**: `pin-ambient-dependency-vouch-composition`,
   `pin-one-file-root-bundle`, `pin-complex-book-source-render`, `pin-plan-root-without-scaffolding`,
   `pin-bundle-map-v0-grammar`, `pin-command-v-load-model`.
6. **`pin-plan-root-without-scaffolding` gains evidence** rather than an answer: cwd analysis now
   demonstrably suffices for every dependency INSIDE the load cwd, and demonstrably cannot serve one
   outside it (`tc-mirroring-refuses-outside-the-load-cwd`). That is the boundary the pin asks
   about, measured on one side.
7. **The two over-coverage candidates in §6**, each owned by another arc.
