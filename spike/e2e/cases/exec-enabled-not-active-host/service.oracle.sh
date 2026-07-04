# service oracle (systemd) — the enabled≠active boundary (task-P/find-1). enable gates
# #enabled, start gates #active (DISTINCT selectors). TWO per-selector probes: is-enabled
# discharges #enabled, is-active discharges #active. This case is ONLY expressible now: a
# single is-active kind-default could not report #enabled holds while #active is absent
# (find-1's under-execute — it would have reported BOTH from is-active, wrongly eliding the
# `start` too on an enabled-but-stopped host).
# command-keyed predict(): the verb selects a different probe per arm (enable→is-enabled,
# start→is-active, disable→is-enabled); annotate the unit operand as `service`.
systemctl__predict() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
      start)   systemctl is-active  -- "$svc" : service:"$svc".active ;;
      disable) systemctl is-enabled -- "$svc" : service:"$svc".enabled! ;;
   esac
}

# THE VOUCH (elide-weld, 24D §3): vouches enable/start (establishes); declines disable + unknown.
systemctl.is_converged() {
   verb=$1; shift
   case $verb in
      enable) systemctl is-enabled -- "$1" >/dev/null 2>&1 ;;
      start)  systemctl is-active  -- "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
