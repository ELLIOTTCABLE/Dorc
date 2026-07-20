#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# package oracle — predict() install/purge; touches() emits a footprint ONLY for install. purge
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 :! sm.dorc.Package:"$pkg"@installed ;;
      esac
   fi
}

apt_get__disturbs() {                              # STRAWMAN footprint — install only (no purge arm)
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) printf '%s\n' "$1" : disturbs sm.dorc.Package ;;
   esac
}

apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
