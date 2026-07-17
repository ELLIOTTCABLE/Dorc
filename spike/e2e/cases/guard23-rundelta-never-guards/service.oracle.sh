#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# service oracle (systemd), enable-only — the run-delta decline fixture. Models ONLY the
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"#enabled ;;
   esac
}
