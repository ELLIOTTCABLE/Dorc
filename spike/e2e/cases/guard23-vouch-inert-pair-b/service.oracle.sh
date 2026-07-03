# service oracle (systemd), enable-only — pair-b: carries the strawman vouch; the pin is
# that this file's ONLY difference from pair-a changes NOTHING in the output.
# ---- STRAWMAN VOUCH SPELLING — NOT DESIGN (rul-guard-license: spelling OPEN; stub).
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
   esac
}
