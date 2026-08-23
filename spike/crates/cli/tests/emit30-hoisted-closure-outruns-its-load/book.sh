#!/bin/sh
# A file-level oracle CONSTANT is not hoisted above a book line that observes the same name
# (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`).
#
# CFG SHAPE: one top-level `Simple` (`hork stage`) whose only argument is a parameter expansion of
# `WOMBAT_ROOT`, standing ABOVE the top-level `.` that binds that name. Three top-level `Simple`s
# follow: the unmodeled wall, and the described `wombat sync` whose vouching body reaches the
# constant through a helper.
#
# WHAT IT OBSERVES: `ran: hork stage unset` — the value the book's own text gives that line. The
# hoist this pin was minted against put `WOMBAT_ROOT=/etc/wombat` ahead of that read, so the line
# received a value the authored program never gave it, and the engine's own counterfactual rail
# called the difference an unattributable delta. The artifact already carries this oracle's bytes AT
# THE `.`, because a book-sited dorc-lang root is placed there by the bundle, so the closure needs no
# hoist at all and the preamble carries only what has nowhere else to be.
hork stage "${WOMBAT_ROOT-unset}"
. ./wombat.dorc.sh
hork provision
wombat sync a.conf
