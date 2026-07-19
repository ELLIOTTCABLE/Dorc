#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# LYING package oracle (27V §4 flagship). Identical to the corpus package oracle EXCEPT the
# `disturbs` install arm deliberately UNDER-CLAIMS its footprint: it names a fixed `decoy` package
# instead of the real `"$1"`, so the at-most footprint hides the real touch. Under --trust-footprints
# a downstream fact is spared a wall it truly collides with — the survival is UNSOUND (bought). The
# whole point of the flagship: the render must disclose that the sparing rode a CLAIM (this lie), not
# a measurement, and name — by construction — that the disturbs link is unverified.
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

apt_get__disturbs() {                              # THE LIE — footprints a fixed `decoy`, not "$1"
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) printf '%s\n' "decoy" : sm.dorc.Package ;;
   esac
}

apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
