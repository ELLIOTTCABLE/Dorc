# service oracle (systemd), enable-only — carries NO converged-vouch (no `is_converged`). Under
# the Part-B elide-weld (24D §3) a converged mutator elides only with a reached vouch; this
# oracle marks none, so even a converged `enable` site RUNS. That vouchlessness is the whole
# point here: the sibling apt oracle's vouch is what lets ITS site elide, and this absence is
# what keeps this site running — the vouch is the only difference (rul-guard-license).
systemctl__predict() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
   esac
}
