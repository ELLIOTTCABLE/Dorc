# service oracle (systemd), enable-only — pair-a: NO vouch (the differential control).
systemctl__predict() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
   esac
}
