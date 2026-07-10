#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# Static reaches() arm: hork's package footprint expands through sm_dorc_Package__reaches()
# to the same-named service coordinate, HITting the converged service-fact's backing ⇒ it
# correctly demotes to run (token-equality alone would wrongly survive it). Traced at plan
# time; no reach probe ships. expected.ran runs BOTH.
hork__predict() {
   verb=$1; shift
   pkg : sm.dorc.Package = "$1"
   case $verb in tune) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg".tuned ;; esac
}
hork__touches() {
   verb=$1; shift
   case $verb in tune) printf '%s\n' "$1" : sm.dorc.Package ;; esac
}

sm_dorc_Package__reaches() {
   printf '%s\n' "$1"    : sm.dorc.Service
}

enablesvc__predict() {
   svc : sm.dorc.Service = "$1"
   systemctl is-enabled -- "$1" >/dev/null 2>&1 : sm.dorc.Service:"$1".enabled
}
enablesvc__is_converged() {
   systemctl is-enabled -- "$1" >/dev/null 2>&1
}
enablesvc__touches() {
   printf '%s\n' "$1" : sm.dorc.Service
}
