# door1-and-form (door-1 && DUAL, charter 20V §4 / note 215): the `&&` direction of the
# cascade, verified symmetric to `||`. `dpkg -s nginx && { … }` runs the block only if the
# guard SUCCEEDS; so a FAILING guard (nginx absent, rc 1) short-circuits `&&` ⇒ the block is
# DEAD. The fold's `eval_and_or` handles `And` and `Or` uniformly (`(And, Some(s)) =>
# Some(s)` — right runs iff left succeeded; here left FAILED ⇒ right dead), and `kill_rec`
# cascades through the brace group exactly as in the `||` cases: BOTH block commands
# (systemctl stop, rm) omit, each needing NO rc-provenance of its own.
#
# This pins the EXTEND-REACH question the charter posed ("verify the fold handles the &&
# direction symmetrically; if not, that is your extend-reach work"): it FOLDS AT BASE — the
# `&&` dual needed no extension (note 215 §2). The guard substitutes to `false` (rc 1, its
# probe-sourced value); the block omits to `{ :; :; }`. Renders `false && { :; :; }`,
# dash-clean, run-set EMPTY (provable by inspection — every leaf is `:`/`false`).
#
# EXEC-ASSERTED via EXIT_RC=1 (slice-1 closed tc-exec-nonzero-exit; was analysis-only): a
# `set -e` book ending in `guard && { … }` whose guard FAILS legitimately exits NON-ZERO
# (the AND-OR list's rc is the failed left's; dash confirms `set -e; false && { … }` does
# NOT abort but the script's final rc is 1). The bare book ITSELF exits 1 under the same
# host — the artifact faithfully reproduces that. The old exec gate treated ANY non-zero
# exit as "errored when run" and FALSE-failed the faithful artifact; the EXIT_RC=1 marker
# now tells the gate to assert rc == 1 EXACTLY, so the empty run-set is EXEC-VERIFIED (the
# apply `false && { :; :; }` logs nothing under mocks and exits 1), not merely golden-proved.
# The probe runs `dpkg -s nginx` against an nginx-ABSENT mock host (gate-1 parity: `site 1
# effect=absent rc=1`). The `&&` RUN pole (guard holds ⇒ block RUNS) is the sibling
# door1-and-form-runs (its live block exits 0). See note 215 §3 strain-and-exit. HOST: nginx
# absent (the guard fails ⇒ `&&` skips the block).
set -e
dpkg -s nginx >/dev/null 2>&1 && { systemctl stop nginx; rm -f /etc/nginx/sites-enabled/default; }
