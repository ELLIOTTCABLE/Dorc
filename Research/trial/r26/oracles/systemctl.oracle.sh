#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# systemctl — two verbs, one unit per call: `enable` (the @enabled cell) and `start` (@active).
# They are separate cells on purpose: `enable` tells you the unit comes back after a reboot,
# `start` tells you it is up right now, and observing either says nothing about the other.
#
# `enable --now` is DECLINED, and that is the most instructive arm in this file. It establishes
# both cells at once, but a verdict body answers through a single exit status, and one status can
# witness exactly one cell. Answering on @active alone would let an active-but-not-enabled unit
# read as converged and skip the enable — the unit would then not survive a reboot. That is a wrong
# yes, so this file refuses the shape and lets the line run.
#
# Known narrowness, stated rather than hidden: `systemctl is-enabled` also exits 0 for `static` and
# `indirect` units, so for those this oracle reads converged. Accepted here because the book's only
# unit is an ordinary enableable one; an oracle written for strangers' books would compare the
# printed state instead of leaning on the status.
#
# Kind: r26.smoke.Service — throwaway, minted for this round only, NOT the stdlib's sm.dorc.*.

systemctl__is_converged() {
   if [ "${1-}" = "" ]; then return 2; fi
   verb="$1"; shift
   case "${1-}" in
   --now)
      printf 'decline unmodeled --now establishes two cells and one exit status witnesses one\n' >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   esac
   if [ "${2-}" != "" ]; then return 2; fi
   if [ "${1-}" = "" ]; then return 2; fi
   if [ "${1#-}" != "$1" ]; then return 2; fi
   svc : r26.smoke.Service = "$1"
   case "$verb" in
   enable)
      systemctl is-enabled --quiet -- "$svc" 2>/dev/null   : r26.smoke.Service:"$svc"@enabled
      ;;
   start)
      systemctl is-active --quiet -- "$svc" 2>/dev/null    : r26.smoke.Service:"$svc"@active
      ;;
   restart|reload|reload-or-restart)
      # A restart is an action, not a state: the unit being up right now says nothing about
      # whether it is running the configuration you just dropped beside it. There is no cell to
      # read back here, on any machine.
      printf 'decline unsound %s: a restart leaves no converged state to read\n' "$verb" >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   *) return 2 ;;
   esac
}
