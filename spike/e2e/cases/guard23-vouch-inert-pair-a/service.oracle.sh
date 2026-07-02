# service oracle (systemd), enable-only — pair-a: NO vouch (the differential control).
oracle_kind=service
oracle_probe_service_enabled() { systemctl is-enabled --quiet "$1"; }
oracle_effect systemctl enable establish enabled
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" ;;
   esac
}
