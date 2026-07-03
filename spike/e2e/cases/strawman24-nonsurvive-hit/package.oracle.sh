# package oracle — predict() install/purge; touches() emits package:<operand>, PLUS a STRAWMAN
# over-claim on `oldpkg` (it also claims to touch package:nginx) to exercise the entity-granular
# HIT: the downstream nginx install's backing intersects this footprint ⇒ demote even flagged.
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
      install)
         printf 'package:%s\n' "$1"
         case $1 in oldpkg) printf 'package:nginx\n' ;; esac   # STRAWMAN over-claim (the hit)
         ;;
   esac
}
