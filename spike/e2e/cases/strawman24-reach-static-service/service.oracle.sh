# strawman24-reach-static-service (24G Stage 5 Part B — the STATIC reaches() arm). A running package
# wall (`hork tune nginx`, footprint package:nginx) precedes a converged SERVICE fact
# (`enablesvc nginx`, backing service:nginx). Token-equality calls package:nginx and service:nginx
# disjoint, so WITHOUT reaches the converged service-fact wrongly SURVIVES. The package owner's
# `package.reaches()` carries a STATIC arm — `printf '%s\n' "$1" : service` (a package may enable its
# same-named unit) — TRACED at plan time (NO host round-trip; ships nothing). The engine expands
# hork's package:nginx footprint through it to service:nginx, which HITs the service backing ⇒ the
# service-fact correctly DEMOTES to run. The static-arm twin of the dynamic flagship: no reach probe
# ships (static arms resolve entirely at the cli). expected.ran runs BOTH.
hork.predict() {
   verb=$1; shift
   pkg : package = "$1"
   case $verb in tune) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".tuned ;; esac
}
hork.touches() {
   verb=$1; shift
   case $verb in tune) printf 'package:%s\n' "$1" ;; esac
}

# THE REACH FUNCTION (24G §4), STATIC arm: a package reaches its same-named unit. Traced at plan
# time (the printf value-flow binds `$1` to the entity), ships nothing to the host.
package.reaches() {
   printf '%s\n' "$1"    : service
}

enablesvc.predict() {
   svc : service = "$1"
   systemctl is-enabled -- "$1" >/dev/null 2>&1 : service:"$1".enabled
}
enablesvc.is_converged() {
   systemctl is-enabled -- "$1" >/dev/null 2>&1
}
enablesvc.touches() {
   printf 'service:%s\n' "$1"
}
