# minimal package oracle (apt/dpkg) — predict() + is_converged() + a PAYLOAD-BOUND touches()
# (24E Stage 4). predict()/is_converged() are unchanged from strawman24-survive-simple; only the
# touches() spelling differs: it derives its footprint via the NATURAL payload-bound idiom
# `dpkg -L "$1" | sed 's|^|file:|'` — a PIPE (24E §14). The parser ACCEPTS it (parse-permissively —
# valid sh degrades, never hard-kills, the kLANG mirror-invariant); the static tracer ⊤s on the
# pipeline (NonPrintfCommand) ⇒ it ESCALATES to host-derivation (24E §2/§4), shipping the whole
# body byte-exact to run on the host. `dpkg -L "$1" | sed` ALONE is now a complete, coherent
# footprint: the engine UNIONS the wall's own establish coordinate (package:$1) into the derived
# footprint (24G §8), so the old boilerplate `printf 'package:%s'` — a decoy the (now-dropped)
# derived-lane coherence check tested INSTEAD of the derivation — is gone.
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

apt-get.touches() {                              # PAYLOAD-BOUND footprint (24E §2/§14): DERIVED via a PIPE
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
   install|purge)
      dpkg -L "$1" | sed 's|^|file:|'            # the NATURAL idiom ALONE: a PIPE ⇒ NonPrintfCommand ⊤ ⇒ escalate; engine unions package:$1 (24G §8)
      ;;
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
