# minimal package oracle (apt/dpkg) — predict() + is_converged() + a PAYLOAD-BOUND touches()
# (24E Stage 4). predict()/is_converged() are unchanged from strawman24-survive-simple; only the
# touches() spelling differs: it derives its footprint via the NATURAL payload-bound idiom
# `dpkg -L "$1" | sed 's|^|file:|'` — a PIPE (24E §14). The parser ACCEPTS it (parse-permissively —
# valid sh degrades, never hard-kills, the kLANG mirror-invariant); the static tracer ⊤s on the
# pipeline (NonPrintfCommand) ⇒ it ESCALATES to host-derivation (24E §2/§4), shipping the whole
# body byte-exact to run on the host. The leading `printf 'package:%s'` emits the wall's own
# establish coordinate (so the coherence check own-establish ⊆ footprint passes — a pure file-level
# cross-kind derivation would fail it; resid-derive-coherence).
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
      printf 'package:%s\n' "$1"                 # the wall's own establish coordinate (coherence)
      dpkg -L "$1" | sed 's|^|file:|'            # the NATURAL idiom: a PIPE ⇒ NonPrintfCommand ⊤ ⇒ escalate
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
