#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# yum provider for the SAME `package` kind (the 17N cross-oracle Seam). Its own check
yum__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) rpm -q "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#installed ;;
         remove) rpm -q "$pkg" >/dev/null 2>&1 :! sm.dorc.Package:"$pkg"#installed ;;
      esac
   fi
}

yum__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) rpm -q "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
