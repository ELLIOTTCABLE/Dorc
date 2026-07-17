#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package oracle (apt/dpkg) — the guard23 INVERTED-VERDICT fixture (23J conv-rc-soundness
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

apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      purge) dpkg-query -W "$1" >/dev/null 2>&1; case $? in 1) return 0 ;; 0) return 1 ;; *) return 2 ;; esac ;;
      *) return 2 ;;
   esac
}
