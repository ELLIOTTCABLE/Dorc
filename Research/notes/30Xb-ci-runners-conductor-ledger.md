# 30Xb — CI runners: conductor ledger

> Tier: conductor ledger (Opus, opened 2026-09-29), a subdoc of `30X` (the testing architecture)
> because that is where the suite's shape lives; this arc adds the GitHub Actions rung beside the
> local gates. Additive sections as the arc proceeds. `[TYPED]` items are the human's rulings;
> `[PROPOSED]` items are the conductor's and carry no ack until one is typed. Authority:
> `IMPLEMENTATION.md`, `spike/CLAUDE.md` and `30X` outrank this file.

## §0 — state

Branch `ai/ci-runners`, worktree `.tmp/trees/ci-runners`, cut from the published `ai/main`. Landed
on the branch: the gate wording (what a gate is, and the task descriptions that promised steps
they do not run) and the cloud-setup hook install. Next: the `ci:` tasks and the workflow files,
once §3's open decisions are typed.

## §1 — the plan

- **`ci-is-not-a-gate`** `[TYPED]` — a gate is what you must pass before you may do something, and
  it checks what YOU changed, through hk's routing. CI is a different thing: clean independent
  runners, each doing one complete, focused job. CI-flavoured tasks are spelled `ci:`; there is no
  `gate:` task for CI and no `ci:all`.
- **`ci-bypasses-local-ceremony`** `[TYPED]` — preflight and the heavy-work lock exist for a shared
  developer machine; a CI job calls the unit under test directly. GitHub reports a job killed by a
  cap or a timeout itself.
- **`ci-yaml-is-first-party-and-thin`** `[TYPED lean]` — GitHub's own YAML and first-party
  actions wherever they serve; no shell scripts. `[PROPOSED]` every step is `actions/checkout`,
  `jdx/mise-action` (which runs `mise install --locked` because `mise.lock` exists), a cache step,
  or `mise run <task>`; no `cargo` or `hk` flags in YAML; nothing branches on a `CI` variable in
  product or test code.
- **`ci-never-reuses-both`** `[TYPED]` — `mise run both` is a local hack for one machine with two
  platforms; CI's platform axis is a job matrix.
- **`ci-one-job-per-lane`** `[TYPED]` — anything that cannot vary by platform, or cannot tell us
  something new about Dorc on a platform, runs on Linux only, one runner per lane.
- **`ci-small-checks-ride-the-primary-run`** `[TYPED]` — the cheap checks (lints, commit
  messages, actionlint, the audit) run early in the primary Rust job, before the build effort they
  can save.
- **The lanes** `[PROPOSED]`:

  | lane | runner | runs | trigger |
  |---|---|---|---|
  | `rust` | ubuntu-24.04 · windows-2025 · macos (best-effort) | Linux first: `ci:commit-messages`, `check` (lints + clippy + actionlint), `audit`; Windows and macOS: `clippy`. Then everywhere: `test`, `test:hooks`, `test:real-tools`, `verify:check` | every push and PR |
  | `floor` | Linux | `test:floor` against pinned dash 0.5.12 and posh 0.14.1 | every push and PR |
  | `kani` | Linux | `verify:kani-setup`, then `ci:kani` | relevant changes only `[TYPED]` |
  | `lean` | Linux | `verify:lean-bootstrap`, then `ci:lean` | relevant changes only `[TYPED]` |
  | `translate` | Linux | `verify:translate-check` | relevant changes only |
  | `assay` | Linux | `ci:assay`: the official tier, sized to fit a hosted job | relevant changes only `[TYPED]` |
  | `livetest` | Linux (Docker) | `livetest` | `[TYPED]` its own job |

- **Triggers** `[TYPED]` — pushes to `ai/main`, and pull requests.
- **Concurrency** `[PROPOSED]` — the `rust` lane cancels a superseded run; the heavy lanes run to
  completion, and a newer push replaces only the pending run, so the latest commit is always tested
  without starving a long lane.

## §2 — typed record, 2026-09-29

- Gate means "run this before you're allowed to …", scoped to what you have done; clear up any
  unclear reference; fix gate descriptions that are actively wrong, without historical language.
- No reuse of `both`; a runner does one complete, focused, sane thing — the point of the matrix.
- `cloud-setup.sh` serves Claude Code containers, a different goal; prefer GitHub YAML and
  first-party actions, avoid shell scripts; fix its missing hook install.
- Preflight is local-only; bypass ceremony not tied to the unit under test.
- The floor test needs both shell versions pinned in CI; the floor versions must be true
  everywhere the test takes place. Encoding them in mise is welcome if it can be done; the
  binaries and their stability matter, not how they are fetched.
- Triggers: `ai/main` and PRs for now.
- The official assay tier is meant to be primarily CI; its last run took 1.5 h; it may be sized
  down to fit hosted runners, up to a point; it, Kani and Lean run on actual changes, not nightly.
- macOS is in scope; if it fails it fails, and it is not the focus.
- Livetest runs in CI as its own job.
- Big lanes that cannot vary by platform run on Linux, one runner each.
- The small checks, commit messages and actionlint included, run early in the primary run.
- NACK `gate:all`: `gate:` is always through hk and filtered. Use `ci:` for CI-specific tasks;
  probably no `ci:all`; a `ci:` version of each task that needs a runner flavour, then one workflow
  entry per task. Perhaps the quick checks combine with the generic Rust tests (the audit too).
- Open to `cargo-binstall` in place of the `aqua:` backend; an hk upgrade if not invasive.
- The project has been greened in a cloud container already; some reds are known (the `specs/`
  buildout). The conductor authors YAML and reads errors; no local verification.
- Standing authorization for this arc: force-push ONLY `ai/`-prefixed branches, ONLY as a leased
  force-push with the remote and both refs spelled in full.

## §3 — open decisions

- **`ask-floor-shell-provisioning`** — how CI (and every other place the floor runs) gets exactly
  dash 0.5.12 and posh 0.14.1.
- **`ask-heavy-lane-routing`** — how "only on relevant changes" is decided: GitHub's own `paths`
  filter per lane (one workflow file per heavy lane) or hk's routing.
- **`ask-ci-task-layering`** — whether a `ci:` task spells its tool invocation itself, or the local
  `:held` twin composes it.
- **`ask-prebuilt-cargo-tools`** — how `cargo-deny`, `cargo-nextest` and the Kani shim install
  without compiling from source.
- **`ask-hk-major-upgrade`** — whether hk 2 lands in this arc.
- **`ask-open-draft-pull-request`** — whether the conductor may open a draft PR from
  `ai/ci-runners` into `ai/main`, which is what triggers runs under the ruled triggers.
