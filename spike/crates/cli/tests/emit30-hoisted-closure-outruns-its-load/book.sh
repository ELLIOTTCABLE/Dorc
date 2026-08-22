#!/bin/sh
# TARGET (`30P:review-adjudication-inputs` `fnd-emission-legality-covers-all-shell-state`, the OPEN
# item it left for the planner lane): a file-level oracle CONSTANT must not be hoisted above a book
# line that observes the same name.
#
# CFG SHAPE: one top-level `Simple` (`hork stage`) whose only argument is a parameter expansion of
# `WOMBAT_ROOT`, standing ABOVE the top-level `.` that binds that name. Three top-level `Simple`s
# follow: the unmodeled wall, and the described `wombat sync` whose vouching body reaches the
# constant through a helper.
#
# THE BURN: `pin_definitions`'s snapshot track hoists every `ClosureDecl` — helper funcdefs AND
# file-level constants alike — verbatim into the preamble above the WHOLE book, with no collision
# check of any kind (the only checks it has, `book_defines_at_top_level`/`book_already_defines`, run
# on the role-body track and read the book's top-level FUNCDEF names). So `WOMBAT_ROOT=/etc/wombat`
# lands ahead of line 12 and `hork stage` receives a value the authored program never gave it. The
# authored program binds the name at line 13 and not before; the artifact binds it at the top. That
# is `pinned-definitions-are-the-artifact's-binding`'s own hazard — a hoist changing which binding is
# live at a book line — for a VARIABLE rather than a funcdef, and nothing guards it.
#
# TARGET BEHAVIOUR: the artifact already carries this oracle's bytes AT THE `.`, because a
# book-sited dorc-lang root is placed there by the bundle. So the closure needs no hoist at all: the
# `in-place` placement value is the answer, and the preamble carries only what has nowhere else to
# be. `ran: hork stage unset` is the observable — the value the book's own text gives that line.
hork stage "${WOMBAT_ROOT-unset}"
. ./wombat.dorc.sh
hork provision
wombat sync a.conf
