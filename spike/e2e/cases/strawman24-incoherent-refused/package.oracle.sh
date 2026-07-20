#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package oracle — predict() install/purge; touches() with a STRAWMAN INCOHERENT arm: for oldpkg
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

apt_get__disturbs() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) case $1 in
         oldpkg) printf 'package:wrongpkg\n' ;;   # STRAWMAN incoherent (omits its own package:oldpkg)
         *) printf '%s\n' "$1" : disturbs sm.dorc.Package ;;
      esac ;;
   esac
}
