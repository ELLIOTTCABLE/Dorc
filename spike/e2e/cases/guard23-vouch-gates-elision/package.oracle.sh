#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# apt/dpkg oracle WITH a converged-vouch — the `apt-get install` line ELIDES.
# The VOUCH is the authored verdict function `apt-get.is_converged()`
# (rul24-vouch-is-verdict-authoring, 24A §1c): under the Part-B elide-weld (24D §3) a converged
# mutator elides ONLY IF a reached vouch licenses it. This oracle carries one; the sibling
# `systemctl` oracle does NOT, so its `enable` RUNS — the vouch is the whole difference.
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

# The verdict function: authoring it IS the vouch (its stripped body is the convergence check).
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg="$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 ;;
      esac
   fi
}
