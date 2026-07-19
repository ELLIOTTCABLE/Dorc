#!/bin/sh
# conduct-bless — the conductor's ONE-SHOT pre-bless verification + golden re-bless.
# (27V Lane C · dispatch d1). BLESS is orchestrator-exclusive (spike/CLAUDE.md: never run by a
# builder / mid-flight); this wraps it so the conductor runs it ONCE, after ALL catalog prose
# edits are finished (amendment-single-bless-confirmed), and reads a single tally.
#
# Does, in order, from spike/: a FRESH `cargo build --workspace`, the full unit suite
# (`cargo test --workspace`), and the foreground e2e — with BLESS=1 by default (re-golden every
# case from the freshly-built binary). Prints ONLY a one-line tally, then `git diff --stat` of the
# goldens the bless changed. Non-interactive; NEVER prompts; fails LOUDLY (nonzero exit + the
# captured tail) on anything real.
#
# Usage:
#   sh e2e/conduct-bless.sh          # build + test + e2e WITH BLESS=1 (the real conductor run)
#   DRY=1 sh e2e/conduct-bless.sh    # same, but e2e runs WITHOUT bless (smoke / dry check — no
#                                    # golden writes). This is the only mode a builder may run.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
spike=$(CDPATH= cd -- "$here/.." && pwd)

log=$(mktemp)
cleanup() { rm -f "$log"; }
trap cleanup EXIT INT TERM

# Run a labelled step, capturing combined output. On failure: print the label, the captured
# tail, and abort with the step's exit code (fail-loud, no swallowing).
step() {
  _label=$1
  shift
  if ( cd -- "$spike" && "$@" ) >"$log" 2>&1; then
    return 0
  fi
  # `$?` must be read in the else-arm, un-negated: `if ! cmd` inverts the status and
  # made a failing step exit 0 (found live, first conductor run).
  _rc=$?
  echo "conduct-bless: FAILED at [$_label] (exit $_rc)" >&2
  tail -40 "$log" >&2
  exit "$_rc"
}

# 1. fresh build (spike/CLAUDE.md: force a fresh build before trusting e2e).
step "build --workspace" mise exec -- cargo build --workspace

# 2. full unit suite.
step "test --workspace" mise exec -- cargo test --workspace
unit=$(awk '/test result: ok\./ { for (i = 1; i <= NF; i++) if ($i ~ /^passed;?$/) s += $(i - 1) }
            END { print s + 0 }' "$log")

# 3. foreground e2e. BLESS=1 by default (orchestrator re-bless); DRY=1 suppresses it (smoke path).
if [ "${DRY:-}" = "1" ]; then
  step "e2e (dry, no bless)" env DORC_E2E_QUIET=1 sh e2e/run.sh
  _e2e_mode="passed"
else
  step "e2e --bless" env BLESS=1 DORC_E2E_QUIET=1 sh e2e/run.sh
  _e2e_mode="blessed"
fi
e2e=$(awk '/e2e round-trips passed/ { print $2 } /^blessed [0-9]+ cases/ { print $2 }' "$log")

# 4. the four pre-commit gates, so the tally's "gates ok" is a run fact, not a claim.
step "fmt --check" mise exec -- cargo fmt --check
step "clippy" mise exec -- cargo clippy --workspace --all-targets -- -D warnings
step "deny" mise exec -- cargo deny check licenses bans sources
step "typos" mise exec -- typos .

echo "conduct-bless: build ok | unit ${unit} passed | e2e ${e2e:-0} ${_e2e_mode} | gates ok"

# The goldens the bless touched (empty in DRY mode — nothing is written).
( cd -- "$spike" && git diff --stat -- e2e/cases e2e/lint-cases )
