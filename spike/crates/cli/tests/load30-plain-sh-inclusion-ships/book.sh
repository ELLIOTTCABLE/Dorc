#!/bin/sh
# TARGET (`30P:mech-acquire-and-ship-plain-sh`, the r30 slice of
# `principle-book-code-source-is-inclusion`): an ordinary sh helper file a book sources is READ,
# recorded as a load occurrence, and MIRRORED beside the published plan — while nothing inside it is
# analyzed (`FORFEITS:forfeit-plain-sh-inclusion-analysis`: no splice, no bindings, the site walls).
#
# The capability is OBSERVED by RUNNING, which is the only way to observe it: the exec gate runs the
# published `plan.sh` from inside its generation ALONE
# (`cli/CLAUDE.md an-artifact-set-runs-from-its-own-generation`), so `plain_helper_step` can only be
# defined there if the `.` found a mirrored `helpers.sh` beside the plan. A generation carrying the
# plan alone dies at that line — a failed `.` is fatal even as the left operand of `||`
# (`floor30-atlas-dot-missing-file-is-fatal`) — and the run set is empty instead.
#
# TARGET RUN SET: `ran: wombat note done`, emitted from inside the included file's own function.
. ./helpers.sh

plain_helper_step
