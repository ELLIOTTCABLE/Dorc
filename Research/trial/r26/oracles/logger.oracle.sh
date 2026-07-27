#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# logger — an oracle whose entire content is one honest refusal, kept so the live run has a wall
# that is attributed rather than anonymous.

logger__is_converged() {
   # Appending a line to syslog is something the system does and forgets. There is no state to
   # read back and compare, on any machine, so "already converged" has no meaning here and never
   # will — this line runs every time, by design.
   printf 'decline unsound logger appends to a log; there is no state to compare\n' >>"${DREP_V1:-/dev/null}"
   return 2
}
