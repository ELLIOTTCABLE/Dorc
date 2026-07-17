#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# apt/dpkg oracle WITH a converged-vouch — the `apt-get install` line ELIDES.
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
   pkg="$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 ;;
      esac
   fi
}
