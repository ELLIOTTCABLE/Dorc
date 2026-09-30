# 30Xb — CI runners: conductor ledger

> Tier: conductor ledger (Opus, opened 2026-09-29), a subdoc of `30X` (the testing architecture)
> because that is where the suite's shape lives; this arc adds the GitHub Actions rung beside the
> local gates. Additive sections as the arc proceeds. `[TYPED]` items are the human's rulings;
> `[PROPOSED]` items are the conductor's and carry no ack until one is typed. Authority:
> `IMPLEMENTATION.md`, `spike/CLAUDE.md` and `30X` outrank this file.

## §0 — state

Folded into `ai/main` by fast-forward; PR #4 carried the runs, and the branch `ai/ci-runners` and
worktree `.tmp/trees/ci-runners` remain for the arc's rest. Landed: the gate wording; the cloud-setup hook install; cargo-deny and
nextest from checksummed release binaries; the `ci:` tasks, composed by the local heavy tasks;
actionlint as an hk step; `assay-lock` routed by assay's own solver path; the `ci` workflow, its
heavy lanes filtered by globs read out of `hk.pkl` at run time; the `floor-shells` workflow, whose
pre-release `floor-shells-dash-0.5.12-posh-0.14.1` is published and which nothing consumes yet.
The first CI results are §5.

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

- **`ask-floor-proves-its-binary`** — how the floor lane shows it ran the pinned binaries.

## §4 — typed record, 2026-09-29 (second sitting)

- CI path filtering is GitHub-side; drift between it and hk is unwanted — research first.
- A task important enough to own a CI lane may take a bare word; names are not to be spread
  through documentation, because they will be hand-tuned afterwards.
- Try `mise lock` with prebuilt (binstall-style) cargo installs.
- The hk 2 upgrade is deferred.
- Pushes from agent sessions: an `ai/` branch onto an `ai/` branch, either unforced or as a leased
  force-push, remote and refs spelled in full — made mechanical in the global push-deny hook.
- The commit-message CI check is punted; this arc carries enough miscellany.
- The floor is the OFFICIAL upstream releases (dash 0.5.12, posh 0.14.1); a distribution's patched
  build is a bug. The local Windows floor runs through WSL, not Cygwin. Building and publishing the
  floor binaries is acked if GitHub makes it easy to publish artifacts that are obviously not a
  release of Dorc, or kept explicitly separate; failing that, they are built and published with
  each semver-minor Dorc release. Building and testing other platforms is valuable, and a
  separate question.

## §5 — typed record and first results, 2026-09-30

- `[TYPED]` `assay-lock` also routes on assay's own code, narrowly: no ordinary internal-tooling
  edit may trigger it.
- `[TYPED]` The pull request: opened with `gh`, a short title, no body.
- `[TYPED]` The floor build is tried in CI and published as described; the release is deletable if
  it pans out badly. Its trigger is a placeholder until semver-minor releases exist, and nothing
  depends on it yet.
- `[TYPED]` macOS stays red; no attempt to fix it.
- Agent pushes: an `ai/` branch onto an `ai/` branch, unforced or leased, remote and refs in full.
- First results, PR #4. Linux and Windows `rust`: green end to end. `livetest`, `translate`:
  green. `kani`: 107 of 107 harnesses green in about twelve minutes. `lean`: mathlib's cache
  fetched, `lake build` green, badges compared. `assay`: correctly not selected (nothing on its
  paths changed). `changes`: the filters rendered from `hk.pkl` on the first run. `floor-shells`:
  static x64 and arm64 builds, published after two fixes (posh's generated autotools files predate
  its `acinclude.m4`, so they are regenerated; a release tag aimed at the pull request's head
  commit got a bare HTTP 500, and the default branch as target did not).
- macOS `rust`, observed only: 19 of 3145 trials red — two e2e looms whose probe records or
  transcripts diverge, and the receipt/staging stores refusing their temp roots ("unsafe staging
  target root"; macOS's temp directory sits under `/var`, a symlink).
- `[TYPED]` The CI assay lane runs `--check --write`: one solve both judges the committed lock and
  writes the measured one, which leaves as the `assay-lock` artifact; only a commit accepts it.
  First official run: 83 commands in 1h39m on three children; 78 green, 3 accepted reds, and 2
  timeouts at the 1800 s CPU cap that the lock already records, so the lane exits 4. Every row of
  the written lock differs from the committed one in `key` alone (results and sizes agree):
  `[TYPED]` the lock format changed, expected.
- `[TYPED]` Folded into `ai/main` by fast-forward, no merge commit.
