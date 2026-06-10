# exec-query-guard-composition (task-D2 / 20B cov-5, the 206-priced recovery door made
# genuine): a `set -e` book with a Query guard. `command -v nginx` is a read-only Query
# whose OWN probed rc (0 = nginx on PATH) is fold-usable (rule-query-validity passes —
# NOTHING mutates upstream of the guard, only `set -e` which is target-state-pure). So:
#   - the fold reads the guard's known rc 0 ⇒ the `|| apt-get install` branch is DEAD
#     ⇒ the install is OMITTED;
#   - the guard itself mutates nothing and its rc is known + `||`-consumed ⇒ it is
#     value-preservingly substituted to its exact stand-in `true`.
# The whole guard line collapses to `true`, UNDER errexit, and the run-set proves it:
# nothing mutating runs (expected.ran is empty). This is the composition cov-5 flagged
# as "composed-but-never-tested" — a genuinely probe-sourced Query rc under `set -e`,
# eliding the install AND substituting the guard exactly. (Contrast a MUTATOR under
# `set -e`, whose ⊤ rc forbids elision — exec-errexit-top-status-runs; a Query rc is
# probe-sourced, so it folds.) HOST: nginx on PATH (the guard holds).
set -e
command -v nginx >/dev/null 2>&1 || apt-get install -y nginx
