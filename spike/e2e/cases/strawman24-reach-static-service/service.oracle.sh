#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# Static disturbance_reaches_only() arm: hork's package footprint expands through sm_dorc_Package__disturbance_reaches_only()
hork__predict() {
   verb=$1; shift
   pkg : sm.dorc.Package = "$1"
   case $verb in tune) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@tuned ;; esac
}
hork__disturbs() {
   verb=$1; shift
   case $verb in tune) printf '%s\n' "$1" : disturbs sm.dorc.Package ;; esac
}

sm_dorc_Package__disturbance_reaches_only() {
   printf '%s\n' "$1"    : disturbs sm.dorc.Service
}

enablesvc__predict() {
   svc : sm.dorc.Service = "$1"
   systemctl is-enabled -- "$1" >/dev/null 2>&1 : sm.dorc.Service:"$1"@enabled
}
enablesvc__is_converged() {
   systemctl is-enabled -- "$1" >/dev/null 2>&1
}
enablesvc__disturbs() {
   printf '%s\n' "$1" : disturbs sm.dorc.Service
}
