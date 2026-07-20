#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# minimal package oracle (apt/dpkg) — predict() + is_converged() + a PAYLOAD-BOUND touches()
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed ;;
      esac
   fi
}

apt_get__disturbs() {                              # PAYLOAD-BOUND footprint (24E §2/§14): DERIVED via a PIPE
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
   install|purge)
      dpkg -L "$1" | sed 's|^|sm.dorc.File:|'            # the NATURAL idiom ALONE: a PIPE ⇒ NonPrintfCommand ⊤ ⇒ escalate; engine unions package:$1 (24G §8)
      ;;
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
