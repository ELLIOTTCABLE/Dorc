#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# service oracle (systemd). enable->#enabled, start->#active (distinct selectors).
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"@enabled ;;
      start)   systemctl is-active  -- "$svc" : sm.dorc.Service:"$svc"@active ;;
      disable) systemctl is-enabled -- "$svc" :! sm.dorc.Service:"$svc"@enabled ;;
   esac
}

systemctl__is_converged() {
   verb=$1; shift
   case $verb in
      enable) systemctl is-enabled -- "$1" >/dev/null 2>&1 ;;
      start)  systemctl is-active  -- "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
