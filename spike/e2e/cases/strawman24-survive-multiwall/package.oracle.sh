# package oracle — predict() install/purge; touches() emits a footprint ONLY for install. purge
# is deliberately UN-footprinted here (no purge arm) so the running `apt-get purge` is a
# footprint-less total wall (contrast strawman24-survive-killwall, where purge IS footprinted).
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

apt-get.touches() {                              # STRAWMAN footprint — install only (no purge arm)
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) printf 'package:%s\n' "$1" ;;
   esac
}
