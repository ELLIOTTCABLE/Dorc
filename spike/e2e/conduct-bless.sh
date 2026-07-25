#!/bin/sh
# conduct-bless — the conductor's ONE-SHOT pre-bless verification + golden re-bless.
# (27V Lane C · dispatch d1). BLESS is orchestrator-exclusive (spike/CLAUDE.md: never run by a
# builder / mid-flight); this wraps it so the conductor runs it ONCE, after ALL catalog prose
# edits are finished (amendment-single-bless-confirmed), and reads a single tally.
#
# Does, in order, from spike/: a FRESH `cargo build --workspace`, then the whole suite
# (`cargo test --workspace`) — which since `288:phase-flat-tree-move` CONTAINS the e2e corpus and
# the loom corpus as two `harness = false` targets, so one run yields all three tallies. In bless
# mode a second, e2e-only pass re-goldens every case from the freshly-built binary. Prints ONLY a
# one-line tally, then `git diff --stat` of the goldens the bless changed. Non-interactive; NEVER
# prompts; fails LOUDLY (nonzero exit + the captured tail) on anything real.
#
# Usage:
#   sh e2e/conduct-bless.sh          # build + test + a BLESS=1 e2e pass (the real conductor run)
#   DRY=1 sh e2e/conduct-bless.sh    # same, minus the bless pass (smoke / dry check — no golden
#                                    # writes). This is the only mode a builder may run.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
spike=$(CDPATH= cd -- "$here/.." && pwd)

# Pre-flight the two tools every step below shells out to. Both have bitten under WSL, and both
# bite EXPENSIVELY without this: `mise` is absent from a non-login shell (so all four steps die
# 127), and a git older than this repo's `relativeWorktrees` extension (git 2.48+) refuses the
# whole repository — which lands on the final golden listing, AFTER a ten-minute green run, as a
# bare fatal. A refusal is worth more than a tail when the environment, not the tree, is wrong.
preflight() {
  _what=$1
  shift
  if "$@" >/dev/null 2>&1; then
    return 0
  fi
  echo "conduct-bless: REFUSING — $_what does not work here:" >&2
  "$@" 2>&1 | sed 's/^/  /' >&2
  exit 2
}
preflight "mise" mise --version
preflight "git in this worktree" git -C "$spike" rev-parse --git-dir

log=$(mktemp)
cleanup() { rm -f "$log"; }
trap cleanup EXIT INT TERM

# Run a labelled step, capturing combined output. On failure: print the label, the captured
# tail, and abort with the step's exit code (fail-loud, no swallowing).
step() {
  _label=$1
  shift
  # The one set-e-safe rc capture: `|| _rc=$?` reads the LEFT side's true status. Both
  # `if ! cmd; then _rc=$?` (negated) and `if cmd; then return; fi; _rc=$?` (reads the
  # if-statement's 0) mis-captured — each found live on a real failing run.
  _rc=0
  ( cd -- "$spike" && "$@" ) >"$log" 2>&1 || _rc=$?
  if [ "$_rc" -ne 0 ]; then
    echo "conduct-bless: FAILED at [$_label] (exit $_rc)" >&2
    tail -40 "$log" >&2
    exit "$_rc"
  fi
}

# 1. fresh build (spike/CLAUDE.md: force a fresh build before trusting e2e).
step "build --workspace" mise exec -- cargo build --workspace

# 2. the whole suite, e2e and looms included.
step "test --workspace" env DORC_E2E_QUIET=1 mise exec -- cargo test --workspace
# Attribute each `test result:` to the target last announced, so the three tiers report
# separately instead of summing into one meaningless number.
tally=$(awk '
  /^ *Running / { target = $0 }
  /test result: ok\./ {
    for (i = 1; i <= NF; i++) if ($i ~ /^passed;?$/) n = $(i - 1)
    if (target ~ /tests[\\\/]e2e\.rs/)        e2e += n
    else if (target ~ /tests[\\\/]looms\.rs/) looms += n
    else                                      unit += n
  }
  END { printf "%d %d %d\n", unit + 0, e2e + 0, looms + 0 }
' "$log")
unit=${tally%% *}
rest=${tally#* }
e2e=${rest%% *}
looms=${rest##* }

# 3. the e2e bless pass (skipped under DRY): re-golden every case from the verified binary.
if [ "${DRY:-}" = "1" ]; then
  _e2e_mode="passed"
else
  step "e2e --bless" env BLESS=1 DORC_E2E_QUIET=1 mise exec -- cargo test -p dorc-cli --test e2e
  _e2e_mode="blessed"
fi

# 4. the four pre-commit gates, so the tally's "gates ok" is a run fact, not a claim.
step "fmt --check" mise exec -- cargo fmt --check
step "clippy" mise exec -- cargo clippy --workspace --all-targets -- -D warnings
step "deny" mise exec -- cargo deny check licenses bans sources
step "typos" mise exec -- typos .

echo "conduct-bless: build ok | unit ${unit} | e2e ${e2e} ${_e2e_mode} | looms ${looms} | gates ok"

# The goldens the bless touched (empty in DRY mode — nothing is written). The runners now
# live beside the cases they drive, so exclude them: this listing is about DATA.
( cd -- "$spike" && git diff --stat -- crates/cli/tests ':!crates/cli/tests/*.rs' )
