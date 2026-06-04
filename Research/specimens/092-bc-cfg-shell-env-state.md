# 092 — specimen class: bc-cfg — shell-execution-environment state the core must model

> AI-generated. Unlike `specimens/090`–`091` (whole-file, via `tools/inline-specimen.sh`), this is an
> **excerpt class**: many tiny real-world snippets, each **commit-pinned + line-anchored** to its
> upstream permalink. Excerpts are verbatim from the round-6 corpus clones at the SHAs listed at the
> foot (so the line-anchors resolve). The `bc-cfg` finding: a class of *bake-into-core* state that lives
> in the **shell execution environment itself** — shell options, cwd, fds, traps — which real scripts
> read, mutate, scope, and restore, and which the analyzer's CFG model must track because it alters
> reachability, path-resolution, and deferred execution. (Round-9 bake-into-core hunt; lens =
> bless-vs-abdicate, `specimens/090`.)

This is neither *system* state (the W5 ambient/transient axis, `plans/099`) nor program variables (the
variable-closure the human keeps warning against) — it is a third thing, and it is **not abdicatable**
(it is the control flow of the book itself). Each excerpt is tagged `[bless]` (a canonical form to
collapse to) or `[hazard]` (a construct the analyzer must fully model or reject — a silent mis-model is
a wrong-skip, TODO §8).

## cfg-options — shell options are introspectable, mutable, scoped ambient state
`errexit` (`set -e`) on/off decides whether a failed command aborts → it is a CFG edge, not a nicety.

`[bless]` the strict-mode preamble `set -euo pipefail` at top-of-script is the near-universal canonical
form (wave-1 GitHub `pipefail` search: leafo/lapis, memorysafety/river, HariSekhon/…). Read it as the
book declaring: *errors abort · unset is an error · pipeline failure is visible · order matters*.

`[hazard]` errexit toggled **conditionally, by introspecting `$-`** — [nvm.sh#L3143-L3145](https://github.com/nvm-sh/nvm/blob/53855417eb66b9c35b732ac39358f1aae3ee1977/nvm.sh#L3143-L3145):
```bash
  if [ "${-#*e}" != "$-" ]; then
    set +e
    local EXIT_CODE
```
The shell's own option-set (`$-`) is read, then `errexit` is disabled for a region (restored later).
Inside it a failing command does **not** abort — different reachability. An analyzer assuming
errexit-throughout mis-models the region → wrong skip.

`[hazard]` other options are bracketed+restored too — [nvm.sh#L4550](https://github.com/nvm-sh/nvm/blob/53855417eb66b9c35b732ac39358f1aae3ee1977/nvm.sh#L4550):
```bash
            set +f; unset IFS # restore variable expansion
```
`noglob` (`f`) and `IFS` toggled around a region (the comment says "restore"). Globbing/word-splitting
state changes how the *next* commands' arguments expand.

## cfg-cwd — cwd is ambient, but subshells *scope* its mutation
`[bless]` "directory of this script", the canonical form — [rwlove/home-ops tools/claude-worktree.sh#L24](https://github.com/rwlove/home-ops/blob/78d35475f9aadac2eb0988cf28fdac3eab6bfff9/tools/claude-worktree.sh#L24):
```bash
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
```
`[bless]` run-in-a-dir without changing the parent's cwd — [rwlove/home-ops tools/check-readme-drift-vs-main.sh#L57](https://github.com/rwlove/home-ops/blob/78d35475f9aadac2eb0988cf28fdac3eab6bfff9/tools/check-readme-drift-vs-main.sh#L57):
```bash
(cd "$TMP_WT" && python3 "$LINTER")
```
Bake-into-core: a `cd`/`export`/assignment/`set` inside `( … )` or `$( … )` does **not** escape to the
parent. The *inverse* of the transient problem — a mutation that looks global (`cd`) but is locally
contained by the subshell. Miss it and every later relative path is mis-resolved.

## cfg-trap — deferred-execution-at-scope-exit; path-dependent when conditional
`[bless]` the canonical transient cleanup — [martinohmann/home-ops …/restic/app/resources/init.sh#L9](https://github.com/martinohmann/home-ops/blob/b0950b56253e3a56fe7d61d255f8fb2ad233cd64/kubernetes/storage/apps/default/restic/app/resources/init.sh#L9):
```bash
trap 'rm -f "$temp_file"' EXIT INT
```
`[hazard]` **conditional** trap registration — [community.docker tests/utils/shippable/shippable.sh#L223](https://github.com/ansible-collections/community.docker/blob/c5831a79f7baaa6d94cb8fe0301d442d672ab8e7/tests/utils/shippable/shippable.sh#L223):
```bash
if [ "${SHIPPABLE_BUILD_ID:-}" ]; then trap cleanup EXIT; fi
```
Whether the EXIT cleanup runs is path-dependent — the analyzer can neither always- nor never-assume the
deferred effect; it is gated by a runtime condition.

## cfg-swallow — `|| true`: best-effort, exit-status-don't-care
`[hazard+signal]` — [community.docker …/shippable.sh#L31](https://github.com/ansible-collections/community.docker/blob/c5831a79f7baaa6d94cb8fe0301d442d672ab8e7/tests/utils/shippable/shippable.sh#L31) · [rwlove …/omada-cert-sync.sh#L96](https://github.com/rwlove/home-ops/blob/78d35475f9aadac2eb0988cf28fdac3eab6bfff9/kubernetes/apps/network/omada-cert-sync/brain/omada-cert-sync.sh#L96):
```bash
    docker rm -f "${container}" || true  # ignore errors
tpeap stop >/dev/null 2>&1 || true
```
Hazard: under `set -e`, `|| true` means "this mutation cannot abort the script" — model never-fails-here
or downstream reachability is wrong. Signal: the author declares the op *failure-tolerant / best-effort*,
and it co-occurs overwhelmingly with naturally-idempotent **remove/stop** ops. Candidate bless:
`cmd || true` = exit-status-don't-care.

## bc-crossCFG lead (task #2)
Every *common* "bounded section mutates-then-restores shared state" case in this corpus is
**shell-internal** — options (`set +e … set -e`), cwd (subshell), globbing (`set +f …`) — all scopeable
and analyzable. The hoped-rare case is the same shape over **system** state (mount/umount, setenforce,
iptables save/restore); none surfaced here. Tentatively supports the "rare" hope (small, nvm-biased
corpus — hold loosely); task #2 hunts system-state counterexamples specifically (each → a tool keyword).

## Ledger addition (bake-into-core)
- **New category:** shell-execution-environment state — options · cwd · fds · traps. Not abdicatable; the analyzer models it.
- **Bless:** `set -euo pipefail` · `"$(cd … && pwd)"` (scoped cwd read) · `( cd X && … )` (scoped run) · `trap '…' EXIT` (cleanup) · `cmd || true` (best-effort).
- **Model-or-reject (hazards):** `$-`-conditional / mid-script option toggles · conditional `trap` · subshell scoping of cwd/options/fds. A silent mis-model is a wrong-skip (TODO §8): acceptance gates on modeling-completeness, not parse-success.

> Sources (round-6 corpus clones, commit-pinned): nvm-sh/nvm @53855417 · rwlove/home-ops @78d35475 · martinohmann/home-ops @b0950b56 · ansible-collections/community.docker @c5831a79. Excerpts verbatim at those SHAs; line-anchors resolve on the permalinks.
