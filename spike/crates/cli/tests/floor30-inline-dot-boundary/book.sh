#!/bin/sh
# floor30-inline-dot-boundary — the MEASUREMENT the flattened artifact form is owed before it may
# ever inline a load-inert child at its `.` position.
#
# `30Ib` §5 row 8 states the debt plainly: textual inlining LOOKS sound for a marked, load-inert
# child (no top-level `return` is representable in one, and a funcdef cannot fail), but it is
# ARGUED and not MEASURED — and the obvious alternatives are already gone, because
# `fnd-loader-function-errexit-diverges` refuted the generated loader function and a subshell kills
# the very definitions the load exists to install. Textual inlining is the only candidate left, so
# it is the one that has to be asked about rather than reasoned about.
#
# THE MANIFEST IS THE ANSWER: each cell prints which shape a real shell actually took, and the
# answer belongs in an `expected.emitted` beside this file — what `dash` and `posh` BOTH said. The
# engine's own answer for the same book is the transcript beside it, so a divergence between the two
# would be a measured fact rather than an argument.
#
# THAT SECTION IS NOT HERE YET, and its absence is deliberate rather than forgotten: the ONE path
# that may write it is `mise run bless:floor`, which is orchestrator-only and needs BOTH floor
# binaries, and git's Windows userland ships no `posh`. Minting it is two acts, in this order —
# create an EMPTY `expected.emitted` here (that file's presence is what opts the case into gate-9),
# then, from WSL, `mise run bless:floor -- floor30-inline-dot-boundary`, which re-measures the
# manifest and commits it together with the transcript from one run so nothing is hand-computed.
# Until then this is an ordinary round-trip case: the manifest is authored, and unmeasured.
#
# CELL 1 — an inert child (a literal assignment and a funcdef) at a plain `.` position, against the
# same bytes written where the `.` was. If these agree, the ordinary inlining is sound for the
# shape v0 admits.
. ./child.sh
printf 'dot mark=%s\n' "$sm_child_mark"
sm_child_say

sm_child_mark=inlined
sm_child_say() { printf 'inline-child\n' ;}
printf 'inline mark=%s\n' "$sm_child_mark"
sm_child_say

# CELL 2 — the SAME child as the left operand of `||`, which is where the two shapes part company
# and why the flattened form cannot be a blind paste: `.` is ONE command, so the `||` covers the
# whole child, while the inlined bytes are N commands and it covers only the last one. Whatever
# these print, they are what an inlining lowering has to preserve or refuse.
. ./failing.sh || printf 'dot caught\n'
printf 'dot continued\n'

sm_fail_mark=one
false || printf 'inline caught\n'
printf 'inline continued\n'
