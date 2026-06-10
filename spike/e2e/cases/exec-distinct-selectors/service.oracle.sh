# service oracle: enable gates #enabled, start gates #active — DISTINCT selectors of
# one service:nginx cell. Neither discharges the other (an is-active verdict must not
# satisfy an unmet #enabled). The honest F-BLESSED shape (task-P/find-1): TWO per-selector
# probes — is-enabled for #enabled, is-active for #active. A multi-selector kind with only
# a kind-default probe is UN-PROBEABLE (both sites run); these per-selector probes make the
# two sites resolvable to DISTINCT bodies (the find-1 under-execute fix).
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
