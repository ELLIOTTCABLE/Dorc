# minimal package oracle (apt/dpkg) — predict() + is_converged() + a PAYLOAD-BOUND touches()
# (24E Stage 4). predict()/is_converged() are unchanged from strawman24-survive-simple; only the
# touches() spelling differs: it reaches a HOST TOOL (apt-manifest) the static evaluate_touches
# cannot resolve (a NonPrintfCommand ⊤) ⇒ it ESCALATES to host-derivation (24E §2/§4) instead of
# emitting statically. The escalating command is a SIMPLE command (the dialect parser rejects the
# pipe/loop a raw `dpkg -L | sed` would need — surfaced 24E-build; a real oracle would ship a
# coordinate-emitting helper like this apt-manifest).
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
      esac
   fi
}

apt-get.touches() {                              # PAYLOAD-BOUND footprint (24E §2): DERIVED, not authored
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install|purge) apt-manifest "$1" ;;        # a host tool ⇒ NonPrintfCommand ⊤ ⇒ escalate
   esac
}

# THE VOUCH (elide-weld, 24D §3): a converged ambient install elides ONLY with a reached vouch.
apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
