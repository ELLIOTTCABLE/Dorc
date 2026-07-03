# service oracle: enable gates #enabled, start gates #active — DISTINCT selectors of
# one service:nginx cell. Neither discharges the other (an is-active verdict must not
# satisfy an unmet #enabled). The honest F-BLESSED shape (task-P/find-1): TWO per-selector
# probes — is-enabled for #enabled, is-active for #active. A multi-selector kind with only
# a kind-default probe is UN-PROBEABLE (both sites run); these per-selector probes make the
# two sites resolvable to DISTINCT bodies (the find-1 under-execute fix).
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
