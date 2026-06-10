# service oracle: enable gates #enabled, start gates #active — DISTINCT selectors of
# one service:nginx cell. Neither discharges the other (an is-active verdict must not
# satisfy an unmet #enabled). A real oracle needs TWO probes (is-enabled AND is-active);
# this scrappy fixture under-probes (notes/193 strain-7 / F-BLESSED) — only `enabled`
# is probed here, and the apply is never executed against this body, only -n'd.
oracle_kind=service
oracle_probe_service() { systemctl is-active --quiet "$1"; }
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
