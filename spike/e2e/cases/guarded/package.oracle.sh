# minimal package oracle (apt/dpkg), lifted statically by dorc.
# command-keyed predict(): the oracle's OWN argparse → inline kind-annotation (the real
# entity-resolution; task-W). Flag-strip (pre- and post-verb), bind the verb, annotate
# the single operand as `package`; the `[ "$2" = "" ]` guard refuses a SECOND operand
# (so `install nginx curl` resolves no probe ⇒ runs — no wrong single-entity elision).
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

# THE VOUCH (elide-weld, 24D §3): vouches install (establish); declines purge + unknown (`*) return 2`).
apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
