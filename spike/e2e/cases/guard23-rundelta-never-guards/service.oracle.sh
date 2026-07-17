#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# service oracle (systemd), enable-only — the run-delta decline fixture. Models ONLY the
# state-shaped verb (`enable` establishes service:<unit>#enabled) and deliberately does NOT
# model `restart`: restart is run-delta (the run is the value), and an oracle DECLINES to
# offer a guard by not vouching a path for it (rul-guard-license).
# ---- STRAWMAN VOUCH SPELLING — NOT DESIGN (rul-guard-license: spelling OPEN; swap-cheap
# ---- stub). Converged-vouch on the ENABLE path only; restart is deliberately unvouched.
systemctl__predict() {
   verb=$1; shift
   svc : sm.dorc.Service = "$1"
   case $verb in
      enable)  systemctl is-enabled -- "$svc" : sm.dorc.Service:"$svc"#enabled ;;
   esac
}
