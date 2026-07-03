# service oracle (systemd). enable->#enabled, start->#active (distinct selectors).
# F-BLESSED honest shape (task-P/find-1): TWO per-selector probes — is-enabled discharges
# #enabled, is-active discharges #active. A single kind-default body cannot soundly observe
# both, so a multi-selector kind with only a kind-default is UN-PROBEABLE (its sites run).
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
