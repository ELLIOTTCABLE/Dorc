#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# service oracle (systemd) — the cross-oracle-scoping fixture's UNVOUCHED oracle B (23C-fd9).
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"#enabled ;;
   esac
}
