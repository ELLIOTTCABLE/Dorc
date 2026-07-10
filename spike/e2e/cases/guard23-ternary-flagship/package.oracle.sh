#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# guard23 flagship package oracle: predict() resolves the entity; authoring is_converged()
# IS the vouch. The ${2-} arity check refuses multi-operand invocations (no probe ⇒ run).
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg".installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg".installed! ;;
      esac
   fi
}

# Unhandled-verb decline style: an unmodeled verb reaches no arm ⇒ no vouch ⇒ run.
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
   esac
}
