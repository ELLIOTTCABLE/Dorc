# package oracle with predict() but NO touches() — an un-footprinted mutator. Under
# --trust-footprints its running install is a TOTAL wall (no footprint to scope it), so the
# downstream converged install demotes exactly as in the honest Stage-1 baseline.
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed! ;;
      esac
   fi
}
