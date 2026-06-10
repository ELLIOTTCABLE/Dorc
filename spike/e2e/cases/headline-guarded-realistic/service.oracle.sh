# service oracle (systemd). enable->#enabled, start->#active (distinct selectors).
# F-BLESSED honest shape (task-P/find-1): TWO per-selector probes — is-enabled discharges
# #enabled, is-active discharges #active. A single kind-default body cannot soundly observe
# both, so a multi-selector kind with only a kind-default is UN-PROBEABLE (its sites run).
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
