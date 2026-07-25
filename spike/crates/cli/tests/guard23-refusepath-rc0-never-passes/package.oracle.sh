#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# package oracle (apt/dpkg) — the guard23 REFUSE-PATH-RC0 fixture (23J conv-rc-soundness
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
      esac
   fi
}

apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   if [ "$2" != "" ]; then return 2; fi
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
