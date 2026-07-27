#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# systemctl — three shapes, one unit per call: `enable`, `start`, and `enable --now`. Two cells
# live here, @enabled and @active; `enable --now` establishes both, so its arm answers for @active
# and discloses the @enabled read that gates it, because one exit status can only witness one cell.
#
# Known narrowness, stated rather than hidden: `systemctl is-enabled` also exits 0 for `static` and
# `indirect` units, so for those this oracle reads converged. Accepted here because the book's only
# unit is an ordinary enableable one; an oracle written for strangers' books would compare the
# printed state instead of leaning on the status.
#
# NB: `test`, not `[` — a bracket test silently voids every mark in this file (see ../README.md).
#
# Kind: r26.smoke.Service — throwaway, minted for this round only, NOT the stdlib's sm.dorc.*.

systemctl__is_converged() {
   command -v systemctl >/dev/null 2>&1 || return 2
   test "$#" -ge 2 || return 2
   verb="$1"; shift
   now=no
   case "${1-}" in
   --now) now=yes; shift ;;
   esac
   test "$#" -eq 1 || return 2
   test "${1#-}" = "$1" || return 2
   svc : r26.smoke.Service = "$1"
   case "$verb" in
   enable)
      case "$now" in
      no)
         systemctl is-enabled --quiet -- "$svc" 2>/dev/null   : r26.smoke.Service:"$svc"@enabled
         ;;
      *)
         systemctl is-enabled --quiet -- "$svc" 2>/dev/null   :? r26.smoke.Service:"$svc"@enabled
         case $? in
         0) ;;
         1) return 1 ;;
         *) return 2 ;;
         esac
         systemctl is-active --quiet -- "$svc" 2>/dev/null    : r26.smoke.Service:"$svc"@active
         ;;
      esac
      ;;
   start)
      systemctl is-active --quiet -- "$svc" 2>/dev/null   : r26.smoke.Service:"$svc"@active
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
