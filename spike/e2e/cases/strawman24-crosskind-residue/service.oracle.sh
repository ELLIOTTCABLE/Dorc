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
