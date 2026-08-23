#!/bin/sh
# THE DOUBLE-CARRY, closed (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`). The
# transcript below is the evidence: `WOMBAT_ROOT=`, `_wombat_dest()` and `wombat__is_converged()`
# each appear EXACTLY ONCE in the apply artifact, in the absorbed bundle at the authored `.`, and
# the guard preamble carries nothing at all.
#
# CFG SHAPE: a top-level bare assignment binding `WOMBAT_ROOT`, then an unmodeled wall, then a
# top-level `.` of a dorc-lang package that binds the SAME name plus a helper and a role body, then
# the described `wombat sync` whose vouching body reaches the constant through that helper. The `.`
# is the whole of its own line, top-level, redirect-free and assignment-free, so the single stream
# absorbs the bundle where the `.` stood (`floor30-inline-dot-boundary`'s measured cell).
#
# WHAT MOVED. `book_already_defines` reads the BOOK's own top-level bytes, and the bundle's bytes
# are not the book's — so the preamble used to emit a SECOND copy of everything this package binds,
# above the whole book. The run-set was right anyway, by luck: sh is last-wins, the absorbed copy
# sits at the authored `.`, and a guard can only ever sit BELOW its definition's load, so within one
# book the absorbed copy always won for guards. What the hoisted copy governed was the book's own
# lines ABOVE the `.` (`emit30-hoisted-closure-outruns-its-load` is that cell), and the exposure the
# luck hid was the MUNGED world: two copies under different names, the guard invoking one and a
# second live definition of the authored name that nothing accounted for.
WOMBAT_ROOT=/srv/local
hork provision
. ./wombat.dorc.sh
wombat sync a.conf
