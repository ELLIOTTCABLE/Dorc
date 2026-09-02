# 30Xb — lane A brief: the `Seams` bundle and the harness binary

> Tier: builder brief DRAFT (Fable, 2026-09-02), written BEFORE `30-reviewA` returned NACK (`30Xa`
> §0). NOT dispatched. A successor conductor re-cuts it against whatever the human's ruling on the
> report changes, fills the two `<tip>` placeholders with the branch tip at dispatch, and only then
> hands it to an Opus builder. Everything below assumes the `30X` design as amended at `633fd954`.

> You are an Opus builder. Do the work yourself; you MUST NOT spawn subagents.

## Safety — autonomous run (frontload; propagate verbatim)

Put these at the top of your reasoning, and at the very top of *every* subagent prompt you write:
- No git mutation outside this worktree; never, ever push. Local commits on this `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't mutate global state (no system packages or system config; worktree-local `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local, mutation-safe. Clock, network, disk, and randomness only through DI seams; correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen in the repo are frozen evidence; they must never be executed. The only sanctioned executor of fixture material is the central e2e runner, `mise run test:e2e` (syntax-checks, and execs only under inert mocks, in a scrubbed environment with a throwaway-sandbox cwd). It rides `mise run test`, so the ordinary suite IS the executor — never hand-run a book, a mock, or a rendered artifact yourself.
- Perpetuate this block, verbatim, to the top of every subagent prompt.

## Step zero — where you are

Your worktree is `C:\Users\ec\Sync\Code\Dorc\.tmp\trees\r30-30X-test-infra-decruft-conductor`, branch `ai/r30-30X-test-infra-decruft-conductor`. It already exists; do NOT create another. Verify first: `git -C <that absolute path> branch --show-current` must print that branch and `git -C … log --oneline -1` must show `<tip>` (the conductor names it at dispatch). If either differs, STOP and report. Every git command you run carries `-C <that absolute path>`; never a bare `cd` before a mutating git command (`worktree-file-access-law`). The primary checkout at `C:\Users\ec\Sync\Code\Dorc` is radioactive: never read, grep, or cite it. Step 0.5: `mise trust` is done on the Windows leg; inside WSL run `wsl --cd <the worktree path> -- mise trust` before your first `mise run both`, and again for `spike/verify/aeneas/mise.toml` if `verify:translate` complains.

## Step one — read before any task material (in this order, fully)

1. `Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` — FIRST. Follow what it says; it may change how you handle parts of this brief, and it tells you when to instruct the conductor to load a review skill. The conductor cannot read it and will not ask what it says.
2. Root `README.md`, `DESIGN.md`; `spike/CLAUDE.md` (whole); `spike/crates/cli/CLAUDE.md` (whole); `spike/crates/receipt/CLAUDE.md`, `spike/crates/receipt-crypto/CLAUDE.md`, `spike/crates/hostsim/CLAUDE.md`.
3. `Research/notes/30X-testing-architecture-seams-sessions-and-seeds.md` — THE design. §2, §3a, §4, §6 and §11 (lane A + the rip list) are yours.
4. `Research/notes/30Xa-test-infra-decruft-conductor-ledger.md` §1–§3, §5 — the rulings typed since 30X and the reconciliation deltas; `30Xc` for the seat table.
5. `.claude/skills/commit/SKILL.md` and root `.gitlabels`.

## The goal, and why

Dorc's suite is being rebuilt around ONE bundle of typed seams (`30X:model-seams-are-one-bundle`): every nondeterministic edge the engine consumes — clock, receipt-id entropy, key entropy, stdout posture, roots, transport, attempt nonce — is a member, and each member independently selects its implementation. A test tier is a ROW of that matrix; a new mechanism is one column added once and every driver inherits it. Today the equivalent is six ad-hoc `DORC_*` environment pins read by the SHIPPED binary, one constant clock in the e2e harness, and edge implementations constructed inline in a 5,400-line `main.rs`. Lane A builds the structure: the bundle, its one parser, the composition root, and a sibling harness binary — so that lanes B–D can attach sessions, seeds and drivers to it without touching the product again.

The invariant this lane is judged on at checkpoint: `30X:inv-division-at-the-narrowest-edge` — the shipped `main.rs` is edge VALUES plus one call; anything with a branch, a parse, or a decision belongs below the seam. And `30X:inv-fixture-state-never-typeable-into-main`: the shipped `dorc` reads NO harness-shaped environment at all. This is the sharpened reading of `spike/CLAUDE.md rul-fixture-identity-never-production` ("anything published" = public interfaces).

## Deliverables (mechanical, testable)

1. **`Seams`** (name strawman; `rul-strawman-formats-no-compat` — rename freely, all sites in one commit) in the `dorc_cli` lib: one struct, one field per seam, each field an enum of implementations. Columns to build NOW (an implementation exists for each): `clock` {Seeded(seed) → ticking from a seed-derived base with a non-zero step · Pinned(ms) · Os} · `receipt_ids` {Seeded · Os} · `key_entropy` {Seeded · Os} · `attempt_nonce` {Seeded · Os} (the `DORC_FIXTURE_NONCE` seat) · `stdout_posture` {Pinned(interactive|kept) · Os (detect)} · `source_match` (whatever `DORC_FIXTURE_SOURCE_MATCH` selects today; Pinned · Os) · `transport` {Os (real ssh) · the fixture interpreter `DORC_TRANSPORT`/`DORC_TRANSPORT_INTERPRETER` select today, renamed as a seam implementation} · `roots` (the platform-root resolution already behind `RootEnvironment`: keep that route, it is already a seam — surface it as a member rather than re-plumbing it). Typed at the composition root; values only cross into the engine (`lib-target-is-a-loom-seam`: VALUES cross the seam, QUERIES do not).
2. **One parser**: `Seams::from_env(&dyn RootEnvironment)` (or a sibling reader trait on the same footing — `30X:loom-seams-are-sh-lines`: ONE parser serves the real process environment now and the modelled session environment in lane B). Spelling: one variable per seam, `DORC_SEAM_<NAME>=<impl>[:<value>]` (strawman), plus a `DORC_SEED=<u64>` umbrella from which every `Seeded` member derives its own seed (seed + a per-seam salt, so members never share a stream). A malformed selection is a typed refusal, never a silent default. Report the final variable table in your report; lanes B–D consume it verbatim.
3. **`compose::run(Seams) -> ExitCode`** extracted from `main.rs`: arg parsing, source acquisition, root resolution, the receipt edge, the engine — everything below the seam, shared byte for byte by both binaries. `bin/dorc.rs` = `exit(compose::run(Seams::os()))`. `bin/dorc-harness.rs` = `exit(compose::run(Seams::from_env(&real_env)))`, and it REFUSES loudly (typed, nonzero, a one-line stderr sentence) when NO seam variable is set, so it can never be mistaken for the product. `dorc-sh` binds equally: check its `main` against the invariant and report.
4. **Seeded entropy, dependency-free** (`30X:bin-seeded-entropy-is-dependency-free`): a small hand-rolled generator over the seed (hostsim's `lcg-only-entropy` posture; a cli-private module is fine — sharing with hostsim is not this lane's question) feeding the EXISTING trait points `ReceiptIdEntropy` and `KeySecretEntropy` (`EntropyReceiptIds::over`, `EntropyKeysetGenerator::over`). Keys and ids become deterministic per (case, seed); keyset files, store names and every rendered identity are stable.
5. **The shipped binary loses ALL SIX harness-shaped reads**: `DORC_FIXTURE_CLOCK_MS`, `DORC_STDOUT_POSTURE`, `DORC_FIXTURE_SOURCE_MATCH`, `DORC_FIXTURE_NONCE`, `DORC_TRANSPORT`, `DORC_TRANSPORT_INTERPRETER` (`30Xa:delta-six-env-pins-not-three`). After this lane, `grep -rn DORC_ spike/crates/cli/src` shows only seam-parser strings reachable from `dorc-harness`. The `cfg!(debug_assertions)` gate on the transport pins goes with them — a seam is selected, never build-profile-gated.
6. **The e2e runner drives the harness**: `Harness::dorc` in `crates/cli/tests/e2e.rs` spawns `dorc-harness` (directly via `CARGO_BIN_EXE_dorc-harness`, or through a `dorc`-named shim dir on PATH — your call, report which; lane B needs the PATH shim regardless) with seams in the environment: a constant run seed for now (lane C flips to varied), the clock seeded with a per-block base offset from the block ordinal and a non-zero step (retiring the one constant that made two publishes in one case share an order token), posture pinned as today. The transcript's replay COMMAND stays `dorc …` (the runner compares it against what it drives — keep that gate honest, do not weaken it).
7. **Goldens byte-identical**: `mise run bless:dry` clean at the end. 30X's claim is that nothing in the corpus renders an id; if a golden turns out to render a CLOCK value under the ticking seed, re-bless that case with the seeded value and LIST it in your report — do not pin it back to the old constant to preserve bytes.

## Invariants you must not break (slugs; read each in `spike/CLAUDE.md` / `cli/CLAUDE.md` / `30X`)

`inv-determinism` (nothing seeded reaches the kernel; the kernel stays a pure function of injected values) · `io-at-edges-only` · `lib-target-is-a-loom-seam` (its "if the lib wants a clock it is on the wrong side" sentence is re-cut by the conductor at close: under 30X the lib HOLDS the edge implementations as seam members, selected at the composition root, and the engine still receives values — keep that shape) · `rul-fixture-identity-never-production` (a `Seeded` implementation must be structurally unreachable from `bin/dorc.rs`: `Seams::os()` is the only constructor it calls, and no `Seeded` variant is constructible from an `Os`-only path) · `rul-attribution-is-controller-minted` and `rul-scratch-root-never-read-from-host` (a seam value never becomes a host-supplied root) · `one-shell-answer` · `rul-strawman-formats-no-compat` (no aliases, no compat for the old pin names — delete them) · `lexical-fences-are-human-ack-instruments` (do NOT mint a grep-shaped test over `main.rs`; the invariant is judged by the conductor on the diff) · `30Xa:rul-minimize-errorloom-changes` (this lane should touch `errorloom` not at all) · `30X` §3a (no new non-loom test, harness, driver, gate, marker, or fixture shape on your own judgment — STOP and raise).

Anything `tc-*`-shaped — a judgment call whose answer changes a cell for another user, phase, or reliability — is FLAGGED in your report, never resolved locally.

## Comment budget

Rip-don't-update: a comment that describes retired shape is deleted, not edited. `///` doc-comments on NEW public items citing the slug they implement are required and not billed. Inline `//` comments: net ≤ 12 lines across the whole lane. Before ending your turn run, from the worktree root, `git diff <tip>..HEAD -- '*.rs' | grep -cE '^\+\s*//[^/!]'` and `git diff <tip>..HEAD -- '*.rs' | grep -cE '^-\s*//[^/!]'`, subtract, and put both numbers and the net in your report; over budget = not done.

## Process

- Commit granularly as you go, `(AI …)` labels per `.gitlabels` (`cli`, `re`, `new`, `rm`, `test`, `typ` are the likely ones); broken intermediate commits are fine (`DORC_KNOWN_BROKEN="<why>"` if the tree will not compile; never `--no-verify`). Commit by explicit pathspec.
- Never pipe a `mise`/`hk` task through `head`/`tail`/`grep`/`Select-*`; use the `-quiet` variants; redirect to a file if output is large.
- Finish with `mise run both gate:full-quiet` FOREGROUND (Windows leg first), then `mise run bless:dry`; both green before you report.
- The conductor's ledger (`30Xa`) is NOT yours to edit; nor `LIVING_STATUS.md`; nor any `CLAUDE.md`.
- If your worktree vanishes or the branch is not what step zero says, STOP and report.

## Report (your final message; the conductor reads only this)

1. The seam table: member · implementations · env spelling · which binary reaches which.
2. The `main.rs` extraction: what `bin/dorc.rs` and `bin/dorc-harness.rs` contain, verbatim (they should be tiny), and where `compose::run` lives.
3. Every deviation from this brief, each as an OPEN item with your reasoning — the conductor re-derives each.
4. Goldens: byte-identical, or the list of re-blessed cases and why.
5. `tc-*` flags; proposed invariant concepts for the steering files (concept + one sentence; the conductor writes the prose).
6. The comment-budget numbers; the gate/bless results; the final commit hash.
7. Anything `AGENTS.for-builders-only.md` told you to relay.
