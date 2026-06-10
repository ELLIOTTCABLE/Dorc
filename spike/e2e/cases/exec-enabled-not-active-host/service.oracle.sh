# service oracle (systemd) — the enabled≠active boundary (task-P/find-1). enable gates
# #enabled, start gates #active (DISTINCT selectors). TWO per-selector probes: is-enabled
# discharges #enabled, is-active discharges #active. This case is ONLY expressible now: a
# single is-active kind-default could not report #enabled holds while #active is absent
# (find-1's under-execute — it would have reported BOTH from is-active, wrongly eliding the
# `start` too on an enabled-but-stopped host).
oracle_kind=service
oracle_probe_service_enabled() { systemctl is-enabled --quiet "$1"; }
oracle_probe_service_active() { systemctl is-active --quiet "$1"; }
oracle_effect systemctl enable establish enabled
oracle_effect systemctl start establish active
oracle_effect systemctl disable kill enabled
# command-keyed check(): the verb selects a different probe per arm (enable→is-enabled,
# start→is-active, disable→is-enabled); annotate the unit operand as `service`.
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" ;;
      start)   systemctl is-active  -- "$svc" ;;
      disable) systemctl is-enabled -- "$svc" ;;
   esac
}
