# minimal package oracle (apt/dpkg) — predict() + touches() (STRAWMAN spellings, 24A §1b).
# predict(): the oracle's OWN argparse -> inline kind-annotation (entity-resolution; task-W).
# touches(): the at-most FOOTPRINT — a third role-sibling, same argparse, that printfs the
# entity-coordinates the verb mutates (one per line, kind:entity). Lifted STATICALLY (never
# shipped/run this stage). install/purge both touch package:<operand>.
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

apt-get.touches() {                              # STRAWMAN footprint spelling (24A §1b)
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install|purge) printf 'package:%s\n' "$1" ;;
   esac
}
