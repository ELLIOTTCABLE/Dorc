# door1-door3-dead-block-folds (door-1 × door-3 d×d host-flip; charter 20V §4 / notes 215 §5
# hunt-C, 21W §4): the disposition-precedence tripwire on the two deadness mechanisms. This is
# the host-FLIP of door1-door3-inner-elides (which pins the outer-LIVE × inner-converged cell):
# here the OUTER guard HOLDS, so the whole `|| { … }` block is DEAD, and fold-Omit PRE-EMPTS the
# inner door-3 mint per leaf (the 21W §4 d×d spine).
#   - OUTER door-1: `dpkg -s nginx` is CONVERGED (nginx present, rc 0) ⇒ the guard holds ⇒ the
#     `|| { … }` branch is DEAD control-flow ⇒ the fold's `kill_rec` marks EVERY leaf beneath the
#     block dead (note 215 §1), each with no rc-provenance of its own.
#   - INNER door-3: `apt-get install -y curl || true` is, in isolation, a door-3 site (the install
#     rc consumed by `|| true` ⇒ StatusInvariant). But the OUTER fold kills it FIRST: a dead leaf
#     is OMITted, not door-3-Replaced. fold-Omit pre-empts the mint — the inner install AND the
#     inner `true` BOTH render `:` (dead), never the door-3 `true` stand-in. This is the precedence
#     the regression tripwire pins: deadness (per-leaf fold) wins over the door-3 mark (per-||-left
#     cfg) on a leaf that is BOTH dead and door-3-marked.
# So the inner install's convergence is IRRELEVANT here (it is dead either way) — contrast
# door1-door3-inner-elides, where the live block makes the inner door-3 mint observable. The two
# cases together bracket the composition: outer-live ⇒ inner door-3 fires; outer-dead ⇒ fold
# pre-empts it.
#
# Renders `true || { : || :; :; }`: the outer guard → `true` (its probed rc 0); the inner install
# AND the inner `true` → `:` (dead, fold-Omit, NOT door-3); the systemctl → `:` (dead). dash-clean,
# UNDER errexit, run-set EMPTY. HOST: nginx present (outer guard holds), curl present (inner
# install converged — but dead-folded, so its convergence does not matter).
set -e
dpkg -s nginx >/dev/null 2>&1 || { apt-get install -y curl || true; systemctl restart sshd; }
