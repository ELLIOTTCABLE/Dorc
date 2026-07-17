#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package oracle with predict() but NO touches() — an un-footprinted mutator. Under
# --trust-footprints its running install is a TOTAL wall (no footprint to scope it), so the
# downstream converged install demotes exactly as in the honest Stage-1 baseline.
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
