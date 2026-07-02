# service oracle (systemd), enable-only — the run-delta decline fixture. Models ONLY the
# state-shaped verb (`enable` establishes service:<unit>#enabled) and deliberately does NOT
# model `restart`: restart is run-delta (the run is the value), and an oracle DECLINES to
# offer a guard by not vouching a path for it (rul-guard-license).
oracle_kind=service
oracle_probe_service_enabled() { systemctl is-enabled --quiet "$1"; }
oracle_effect systemctl enable establish enabled
# ---- STRAWMAN VOUCH SPELLING — NOT DESIGN (rul-guard-license: spelling OPEN; swap-cheap
# ---- stub). Converged-vouch on the ENABLE path only; restart is deliberately unvouched.
oracle_vouch_converged='systemctl enable'
systemctl__check() {
   verb=$1; shift
   svc : service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : service:"$svc".enabled ;;
   esac
}
