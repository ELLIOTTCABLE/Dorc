# service oracle: `systemctl enable` gates service:<unit>#enabled. NO touches() — the enable is
# the downstream SURVIVOR here, not a wall, so it needs only a predict() (probe + establish).
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
