#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# service oracle (systemd), enable-only — carries NO converged-vouch (no `is_converged`). Under
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"#enabled ;;
   esac
}
