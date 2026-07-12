#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# Survival-tier package oracle.
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#installed! ;;
      esac
   fi
}

apt_get__disturbs() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install|purge) printf '%s\n' "$1" : sm.dorc.Package ;;
   esac
}

# Vouches install only; purge (a kill) and unknown verbs decline via the explicit `*) return 2`
# — a literal `return 0` here would VOUCH, so the catch-all answers can't-say on purpose.
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
