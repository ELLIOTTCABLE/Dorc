# 30Xe — lane A completion brief (after checkpoint A)

> Tier: builder brief (Fable, 2026-09-02). Lane A (`30Xb`) is BUILT to `4ee6aeca` and green on the
> Windows gate leg; its builder stopped at its budget with three items open. This is the SMALL
> completion remit; the rulings it implements are `30Xa` §2a. Finish, verify, report — nothing
> more. The conductor names the branch tip in the dispatch message wherever `<tip>` appears.

> You are an Opus builder. Do the work yourself; you MUST NOT spawn subagents.

## Safety — autonomous run (frontload; propagate verbatim)

Put these at the top of your reasoning, and at the very top of *every* subagent prompt you write:
- No git mutation outside this worktree; never, ever push. Local commits on this `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't mutate global state (no system packages or system config; worktree-local `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local, mutation-safe. Clock, network, disk, and randomness only through DI seams; correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen in the repo are frozen evidence; they must never be executed. The only sanctioned executor of fixture material is the central e2e runner, `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`, so the ordinary suite IS the executor — never hand-run a book, a mock, or a rendered artifact yourself.
- Perpetuate this block, verbatim, to the top of every subagent prompt.

## Step zero — where you are

Your worktree is `C:\Users\ec\Sync\Code\Dorc\.tmp\trees\r30-30X-test-infra-decruft-conductor`, branch `ai/r30-30X-test-infra-decruft-conductor`. It already exists; do NOT create another. Verify: `git -C <that absolute path> branch --show-current` prints that branch; `git -C … rev-parse HEAD` prints `<tip>`; `git -C … status --short` is empty. Any mismatch: STOP and report. Every git command carries `-C <that absolute path>`; never a bare `cd` before a mutating git command. The primary checkout `C:\Users\ec\Sync\Code\Dorc` and every other worktree are radioactive: never read, grep, cite, or run anything there. Step 0.5: inside WSL run `wsl --cd <the worktree path> -- mise trust` before your first `mise run both` (and again for `spike/verify/aeneas/mise.toml` if `verify:translate` complains).

## Step one — read before any task material (in this order)

1. `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` — FIRST. Follow it; it tells you when to instruct the conductor to load a review skill. The conductor cannot read it and will not ask what it says.
2. Root `README.md`, `DESIGN.md`; `spike/CLAUDE.md` (whole); `spike/crates/cli/CLAUDE.md` (whole).
3. `Research/notes/30X-testing-architecture-seams-sessions-and-seeds.md` §2, §3a, §4, §5 (`loom-syntax-grants-no-production-authority` especially), §11 lane A.
4. `Research/notes/30Xa-test-infra-decruft-conductor-ledger.md` §1, §2a (the rulings you implement), §3; `Research/notes/30Xb-lane-a-seams-and-harness-brief.md` (what lane A was told to build).
5. Then the code lane A built: `spike/crates/cli/src/seam.rs`, `spike/crates/cli/src/bin/dorc-harness.rs`, the `Harness` driver in `spike/crates/cli/tests/e2e.rs`, and the harness spawns in `spike/crates/cli/tests/durable_route.rs` / `recorded_facts_route.rs`. Read the lane's commits `1446838e..4ee6aeca` (`git -C … log --stat`) to see what moved.

## Deliverables (all four; small)

1. **`fix-roots-pinned-carries-a-path`** (`30Xa:rul-roots-pinned-is-a-literal`). `RootsSeam::Pinned` carries an ABSOLUTE, runner-owned directory: `DORC_SEAM_ROOTS=pinned:<absolute dir>` (spelling strawman — `rul-strawman-formats-no-compat`, rename freely, no compat with the bare `pinned`). Resolution for `Pinned` derives the config/state roots under that directory and NEVER consults `APPDATA`, `LOCALAPPDATA`, `HOME`, or `XDG_*`; a `pinned` with no path is a typed refusal. `Os` stays the platform resolution and stays unnameable from `HarnessSeams`. The e2e runner and both batteries pass their per-case throwaway root through this seam. Acceptance: with `HOME`/`APPDATA`/`LOCALAPPDATA` unset in the harness environment, every harness-driven test still writes where it did.
2. **`build-the-session-scrub`** (`30X:loom-syntax-grants-no-production-authority`; `30Xb` deliverable 6, unbuilt). Every harness spawn from the runner and from the two batteries starts from `env_clear()` and receives EXACTLY: `PATH` (the shim/mocks path the runner already computes, plus only what the OS needs to execute a binary — on Windows measure whether `SystemRoot`, `ComSpec`, `PATHEXT` are required and report which), the `DORC_SEAM_*` / `DORC_SEED` variables, and nothing else: no inherited credential variables, no `HOME`/`APPDATA`. The runner's OWN process keeps its environment — `DORC_E2E_REAL_TOOLS`, `DORC_E2E_FLOOR_SHELLS`, `BLESS`, `BLESS_FLOOR`, `DORC_E2E_QUIET` are read by the runner, never by the harness; do not scrub the runner. The artifact-execution rail (`env -i`, mocks PATH, throwaway cwd, `umask 022`) is untouched.
3. **`verify-shipped-binary-liveness`** (the bounded remit `30X:inv-division-at-the-narrowest-edge` leaves above the seam). Confirm that at least one existing test spawns the SHIPPED `dorc` (`env!("CARGO_BIN_EXE_dorc")` — `receipt_route.rs` has two such spawns) with the platform variables pointed at a sandbox and asserts OS liveness as a RELATION: two invocations mint different receipt ids, or a keyset lands under the sandboxed platform root. If one exists, cite it (file and test name) in your report and change nothing. If none does, add exactly ONE `#[test]` to `durable_route.rs` asserting that relation — state and relations only, never a render golden (`30X` §3a class 6) — and no other test, harness, gate, marker, or fixture shape.
4. **`run-the-owed-checks`**: `mise run both gate:full-quiet` FOREGROUND, Windows leg first, then the WSL leg (it has never run over lane A; the `#[cfg(unix)]` keyset-permission tests and the umask shim live only there); then `mise run bless:dry`. Both green. Report that the three re-blessed whygallery transcripts are byte-stable under `bless:dry`.

Do NOT edit `30Xa`, `30X`, any `CLAUDE.md`, or anything under `Research/quarantine-DO-NOT-READ/` beyond the step-one read.

## Invariants that bind (slugs; read them in `spike/CLAUDE.md`, `cli/CLAUDE.md`, `30X`)

`inv-determinism` · `io-at-edges-only` · `rul-fixture-identity-never-production` (the TYPE is the fence: no production arm reachable from `HarnessSeams`; never a runtime check standing in for a missing variant) · `30X:loom-syntax-grants-no-production-authority` · `30X:rul-seam-columns-are-conductor-ruled` (no new column; the roots change is a variant's payload, not a column) · `30X` §3a (no new non-loom test beyond deliverable 3's single conditional witness; STOP and raise anything else) · `lexical-fences-are-human-ack-instruments` (no grep-shaped test for the scrub — the type and the runner are the mechanism) · `rul-strawman-formats-no-compat` · `30Xa:rul-minimize-errorloom-changes` (touch `errorloom` not at all) · `one-shell-answer`. Anything `tc-*`-shaped is FLAGGED in your report, never resolved locally.

## Comment budget

Rip-don't-update. `///` doc-comments on new public items (citing their slug) are required and not billed. Inline `//`: net ≤ 6 over the lane. Before ending run `git diff <tip>..HEAD -- '*.rs' | grep -cE '^\+\s*//[^/!]'` and `git diff <tip>..HEAD -- '*.rs' | grep -cE '^-\s*//[^/!]'`; report both and the net.

## Process

Granular `(AI …)` commits by pathspec (`fix`, `cli`, `test`, `sec` are the likely labels); never `--no-verify`; never pipe a `mise`/`hk` task through `head`/`tail`/`grep`/`Select-*` — use the `-quiet` variants and redirect to a file if large. If your worktree vanishes or the branch is wrong, STOP and report. Keep the whole lane well inside one context: if you approach the point where you would need to compact, commit what you have, write the report, and stop.

## Report (your final message)

1. What each of the four deliverables did; the exact environment the harness now receives (the variable list, per platform).
2. Every deviation as an OPEN item with reasoning.
3. `tc-*` flags; proposed invariant concepts (concept + one sentence).
4. Gate results for both legs; `bless:dry` result; the comment numbers; the final commit hash.
5. Anything `AGENTS.for-builders-only.md` told you to relay.
