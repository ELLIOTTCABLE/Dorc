#!/bin/sh
# THE DOUBLE-CARRY, pinned as it stands (`plan/CLAUDE.md pinned-definitions-are-the-artifact's-binding`
# — the ALREADY-IN-PLACE rule, defeated by a route it predates). GREEN, deliberately: the transcript
# below is the evidence, and the emission planner is what moves it.
#
# CFG SHAPE: a top-level bare assignment binding `WOMBAT_ROOT`, then an unmodeled wall, then a
# top-level `.` of a dorc-lang package that binds the SAME name plus a helper and a role body, then
# the described `wombat sync` whose vouching body reaches the constant through that helper. The `.`
# is the whole of its own line, top-level, redirect-free and assignment-free, so the single stream
# absorbs the bundle where the `.` stood (`floor30-inline-dot-boundary`'s measured cell).
#
# WHAT THE TRANSCRIPT SHOWS: `WOMBAT_ROOT=`, `_wombat_dest()` and `wombat__is_converged()` each
# appear TWICE in the apply artifact — once in the hoisted guard preamble and once in the absorbed
# bundle. ALREADY-IN-PLACE asks `book_already_defines`, which reads the BOOK's own top-level bytes;
# the bundle's bytes are not the book's, and `pin_definitions` is not told the artifact will carry
# them. So "the EMITTED preamble never carries two same-named funcdefs" holds against the route it
# was written for and not against the one the bundling added.
#
# WHY THE RUN-SET IS STILL RIGHT HERE, and why that is luck rather than safety: sh is LAST-WINS, and
# the absorbed copy sits at the authored `.` position, so it rebinds every name ahead of the guard
# below it. A guard can only ever sit BELOW its definition's load — positional visibility is what
# licenses the vouch — so within one book the absorbed copy always wins for guards. What the hoisted
# copy governs is the book's own lines ABOVE the `.`, and that is where it bites:
# `emit30-hoisted-closure-outruns-its-load` is that cell, and it is XFAIL.
#
# THE OTHER EXPOSURE, unreached by this corpus: when the preamble MUNGES the role body (a plural or
# defensive-emission world), the two copies carry DIFFERENT names — the guard invokes the munged
# one and the absorbed one is a second, live definition of the authored name that nothing accounts
# for.
WOMBAT_ROOT=/srv/local
hork provision
. ./wombat.dorc.sh
wombat sync a.conf
