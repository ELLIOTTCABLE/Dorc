# minimal package oracle (apt/dpkg) — predict() + is_converged() + touches() (unchanged from
# strawman24-survive-simple), PLUS the 24F Stage 5 identity role: `package.resolve()`. The resolver
# is keyed by KIND (`package`), not the command word (corr-kind-keying §10 — the kind-owner holds
# the nouns): it prints the CANONICAL package for an entity via a provides-resolution
# (`dpkg-query -W -f '${Package}'`), falling back to the name itself. It ships strip-only to the
# probe lane and runs read-only per coordinate; the engine canonicalizes both footprint and backing
# coords through it before disjoint (the same self-vouch tier as its siblings — authoring IS the vouch).
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

apt-get.touches() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in install) printf 'package:%s\n' "$1" ;; esac
}

# THE VOUCH (elide-weld, 24D §3): a converged ambient install elides ONLY with a reached vouch.
apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in install) dpkg-query -W "$1" >/dev/null 2>&1 ;; *) return 2 ;; esac
}

# THE RESOLVER (24F §3): the package kind's canonicalizer. `nginx-full` is a provides-alias of
# `nginx`, so both resolve to the SAME canonical ⇒ the closure detects the alias.
package.resolve() {
   dpkg-query -W -f '${Package}\n' -- "$1" 2>/dev/null || printf '%s\n' "$1"
}
