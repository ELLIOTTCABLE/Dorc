# minimal package oracle (apt/dpkg), lifted statically by dorc.
# command-keyed predict(): flag-strip (pre- and post-verb), bind the verb, annotate the
# single operand as `package`; the `[ "$2" = "" ]` guard refuses a SECOND operand.
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
