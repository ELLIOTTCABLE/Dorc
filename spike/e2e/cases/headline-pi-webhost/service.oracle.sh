#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# service oracle (systemd). enable->#enabled, start->#active (distinct selectors).
# F-BLESSED honest shape (task-P/find-1): TWO per-selector probes — is-enabled discharges
# #enabled, is-active discharges #active. A single kind-default body cannot soundly observe
# both, so a multi-selector kind with only a kind-default is UN-PROBEABLE (its sites run).
# command-keyed predict(): the verb selects a different probe per arm (enable→is-enabled,
# start→is-active, disable→is-enabled); annotate the unit operand as `service`.
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"#enabled ;;
      start)   systemctl is-active  -- "$svc" : sm.dorc.Service:"$svc"#active ;;
      disable) systemctl is-enabled -- "$svc" :! sm.dorc.Service:"$svc"#enabled ;;
   esac
}
