# door1-door3-inner-runs (door-1 × door-3 d×d composition; charter 20V §4 / notes 215 §5/§7
# hunt-C, 213 door-3): the THIRD cell of the door-1 × door-3 bracket — outer guard LIVE × inner
# mutator DIVERGED-runs. The two siblings pin the other cells: door1-door3-inner-elides (outer
# LIVE × inner CONVERGED ⇒ the inner door-3 mints, install → `true`) and door1-door3-dead-block-
# folds (outer DEAD ⇒ fold-Omit pre-empts the inner door-3 per leaf). Here the outer block is LIVE
# AND the inner install DIVERGES, so the inner door-3 site RUNS for real — the cell note 215 §7
# left for crosscheck.
#   - OUTER door-1: `dpkg -s nginx` is DIVERGED (nginx absent, rc 1) ⇒ the guard fails ⇒ the
#     `|| { … }` block is LIVE (the `||` fires). The fold does NOT fold the block away (its known
#     rc 1 keeps the branch reachable); the guard still substitutes to its probe-sourced `false`.
#   - INNER door-3: `apt-get install -y curl || true` is a live door-3 site — the install rc is
#     consumed by `|| true` ⇒ `StatusInvariant` (never blocks). But door-3 clears ONLY the Status
#     channel; the Effect channel still gates (note 213 / door3-or-true-diverged-runs). curl is
#     DIVERGED (absent) ⇒ the convergence check REFUSES the elision license ⇒ the install RUNS
#     verbatim. The `|| true` swallows its rc under `set -e` (the left of `||` is errexit-exempt),
#     so a non-zero install does not abort the book.
# This is the proof that the two doors compose WITHOUT either masking the other's gate: door-1
# keeps the block live (rc-keyed reachability), and within it door-3 still defers to Effect (it is
# not an elision relaxation — Status-clear ≠ license). Contrast door1-door3-inner-elides, where
# the SAME live block lets the inner door-3 MINT because curl converged; the only delta is the
# inner install's host-state.
#
# Renders `false || { apt-get install -y curl || true; systemctl restart sshd; }`: the outer guard
# → `false` (its probed rc 1); the inner install RUNS verbatim (diverged ⇒ Effect gates); the
# inner `|| true` verbatim; the systemctl runs. dash-clean, UNDER errexit, exits 0. run-set:
# `apt-get install -y curl` then `systemctl restart sshd` (book order; `false`/`true` are
# builtins). HOST: nginx absent (outer guard fails), curl absent (inner install diverged ⇒ runs).
set -e
dpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }
