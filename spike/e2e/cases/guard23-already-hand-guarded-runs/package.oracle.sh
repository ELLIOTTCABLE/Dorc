# minimal package oracle (apt/dpkg), lifted statically by dorc — the guard23-* fixture
# variant: identical to the corpus-standard package oracle PLUS the strawman vouch below.
oracle_kind=package
oracle_probe_package() { dpkg-query -W "$1" >/dev/null 2>&1; }
oracle_effect apt-get install establish installed
oracle_effect apt-get purge kill installed
# ---- STRAWMAN VOUCH SPELLING — NOT DESIGN (rul-guard-license: the vouch's concrete sh
# ---- spelling is OPEN; this inert assignment is a swap-cheap stub, and pins built on it
# ---- pin BEHAVIOUR, never this spelling). It stands in for the author's converged-vouch
# ---- on the install path of the check below: "when my check's install path reports
# ---- converged, I judge re-running `apt-get install` at that site skippable; whatever
# ---- it would still do is noise I know of, or residue I accept." A fallible, attributed
# ---- judgment (claimed-tier) — never a fact; it licenses guards at THIS command's sites
# ---- only and never enters the fact-plane (rul-guard-license).
oracle_vouch_converged='apt-get install'
# command-keyed check(): the oracle's OWN argparse → inline kind-annotation (the real
# entity-resolution; task-W). Flag-strip (pre- and post-verb), bind the verb, annotate
# the single operand as `package`; the `[ "$2" = "" ]` guard refuses a SECOND operand
# (so `install nginx curl` resolves no probe ⇒ runs — no wrong single-entity elision).
apt_get__check() {
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
