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

# THE VOUCH (elide-weld, 24D §3): vouches enable/start (establishes, per-selector probes);
# declines disable (a KILL) + unknown verbs via `*) return 2`.
systemctl.is_converged() {
   verb=$1; shift
   case $verb in
      enable) systemctl is-enabled -- "$1" >/dev/null 2>&1 ;;
      start)  systemctl is-active  -- "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
