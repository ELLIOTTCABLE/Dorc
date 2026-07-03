# package oracle — predict() install/purge; touches() with a STRAWMAN INCOHERENT arm: for oldpkg
# it emits package:wrongpkg (NOT its own package:oldpkg), so the establish coordinate is not ⊆ the
# footprint ⇒ the coherence check refuses it. nginx stays coherent (emits its own package:nginx).
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

apt-get.touches() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) case $1 in
         oldpkg) printf 'package:wrongpkg\n' ;;   # STRAWMAN incoherent (omits its own package:oldpkg)
         *) printf 'package:%s\n' "$1" ;;
      esac ;;
   esac
}
