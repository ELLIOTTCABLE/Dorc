#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# minimal service oracle (systemd), lifted statically by dorc.
# This book only `enable`s (gating #enabled); the predict's `enable` arm probes with
# is-enabled, discharging #enabled — the correct, mismatch-free shape for the selector
# it actually uses.
# command-keyed predict(): the verb selects a different probe per arm (enable→is-enabled,
# start→is-active, disable→is-enabled); annotate the unit operand as `service`.
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"#enabled ;;
      start)   systemctl is-active  -- "$svc" ;;
      disable) systemctl is-enabled -- "$svc" :! sm.dorc.Service:"$svc"#enabled ;;
   esac
}

# THE VOUCH (elide-weld, 24D §3): vouches enable/start (establishes); declines disable + unknown.
systemctl__is_converged() {
   verb=$1; shift
   case $verb in
      enable) systemctl is-enabled -- "$1" >/dev/null 2>&1 ;;
      start)  systemctl is-active  -- "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
