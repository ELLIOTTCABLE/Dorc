# yum provider for the SAME `package` kind (the 17N cross-oracle Seam). Its own check
# carries the same `package` annotation — the kind name is the shared cross-oracle anchor,
# so both providers' converged installs elide against one kind. notes/199 cluster-E.
# command-keyed predict(): yum argparses like apt-get here (verb then operand), same
# `package` kind — the cross-oracle Seam (the kind name is the shared anchor).
yum__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then
      case $verb in
         install) rpm -q "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
         remove) rpm -q "$pkg" >/dev/null 2>&1 : package:"$pkg".installed! ;;
      esac
   fi
}
