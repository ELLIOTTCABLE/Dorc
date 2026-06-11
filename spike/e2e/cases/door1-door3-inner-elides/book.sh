# door1-door3-inner-elides (door-1 × door-3 COMPOSITION, charter 20V §4 / note 215 §5 hunt-C):
# a door-1 block whose inner command is itself a door-3 `|| true` site. The mechanisms are
# INDEPENDENT and compose cleanly:
#   - OUTER door-1: `dpkg -s nginx` is DIVERGED (nginx absent, rc 1) ⇒ the `|| { … }` block is
#     LIVE (the guard fails ⇒ the `||` fires). So door-1 does NOT fold the block away.
#   - INNER door-3: within the live block, `apt-get install -y curl || true` has its install rc
#     consumed by `|| true` ⇒ `StatusInvariant` (never blocks); curl is CONVERGED (rc 0 via the
#     establish probe) ⇒ the install MINTS its elision (door-3 / note 213) and is REPLACED by
#     `true`, even though it sits inside a live door-1 block.
# This is the cross-door composition the charter's hunt-list names. door-1's deadness is per-leaf
# (the fold's `kill_rec`); door-3's StatusInvariant is per-`||`-left (cfg `lower_and_or`) — they
# do not interfere. When the outer block is LIVE, the inner door-3 leaf is reachable and its mark
# applies normally.
#
# Renders `false || { true || true; systemctl restart sshd; }`: the outer guard → `false` (its
# probed rc 1); the inner install → `true` (door-3 StatusInvariant license); the inner `true` rhs
# verbatim; the systemctl runs. dash-clean, exits 0. run-set: `systemctl restart sshd` ONLY (the
# inner install elided, `true` is a builtin). HOST: nginx absent (outer guard fails), curl
# installed (inner install converged).
set -e
dpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }
