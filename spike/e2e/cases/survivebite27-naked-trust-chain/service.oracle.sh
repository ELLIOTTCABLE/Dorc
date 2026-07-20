#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# HONEST service oracle: `systemctl start` gates Service:<unit>#active. It backs the downstream
# elision the lying certsync footprint spares — a DIFFERENT tool's fact, cross-kind from CertBundle.
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      start)   systemctl is-active  -- "$svc" : sm.dorc.Service:"$svc"@active ;;
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"@enabled ;;
   esac
}

systemctl__is_converged() {
   verb=$1; shift
   case $verb in
      start)  systemctl is-active  -- "$1" >/dev/null 2>&1 ;;
      enable) systemctl is-enabled -- "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
