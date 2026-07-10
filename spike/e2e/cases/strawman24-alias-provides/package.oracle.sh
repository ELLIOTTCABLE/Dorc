#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# Kind-owner resolver case: sm_dorc_Package__resolve() is keyed by KIND — its NAME is the
# kind's forward-munge. nginx-full provides-resolves to nginx's canonical ⇒ alias detected.
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg".installed ;;
      esac
   fi
}

apt_get__touches() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in install) printf '%s\n' "$1" : sm.dorc.Package ;; esac
}

apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in install) dpkg-query -W "$1" >/dev/null 2>&1 ;; *) return 2 ;; esac
}

sm_dorc_Package__resolve() {
   dpkg-query -W -f '${Package}\n' -- "$1" 2>/dev/null || printf '%s\n' "$1"
}
