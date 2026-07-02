# service oracle (systemd), enable-only — pair-b: carries the strawman vouch; the pin is
# that this file's ONLY difference from pair-a changes NOTHING in the output.
oracle_kind=service
oracle_probe_service_enabled() { systemctl is-enabled --quiet "$1"; }
oracle_effect systemctl enable establish enabled
# ---- STRAWMAN VOUCH SPELLING — NOT DESIGN (rul-guard-license: spelling OPEN; stub).
oracle_vouch_converged='systemctl enable'
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" ;;
   esac
}
