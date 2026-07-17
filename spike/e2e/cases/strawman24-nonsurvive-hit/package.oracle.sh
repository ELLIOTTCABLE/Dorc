#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package oracle — predict() install/purge; touches() emits package:<operand>, PLUS a STRAWMAN
# over-claim on `oldpkg` (it also claims to touch package:nginx) to exercise the entity-granular
# HIT: the downstream nginx install's backing intersects this footprint ⇒ demote even flagged.
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 :! sm.dorc.Package:"$pkg"#installed ;;
      esac
   fi
}

apt_get__disturbs() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install)
         printf '%s\n' "$1" : sm.dorc.Package
         case $1 in oldpkg) printf 'package:nginx\n' ;; esac   # STRAWMAN over-claim (the hit)
         ;;
   esac
}
