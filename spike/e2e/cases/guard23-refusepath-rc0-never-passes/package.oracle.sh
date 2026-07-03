# package oracle (apt/dpkg) — the guard23 REFUSE-PATH-RC0 fixture (23J conv-rc-soundness
# facet (ii) / hz-refusepath). The predict()'s `[ "$2" = "" ]` arity gate is the corpus refuse
# idiom: on a MULTI-operand invocation the `if` is false and the whole predict RETURNS 0 — the
# rc-0-on-refuse hazard. The VOUCH is the authored verdict function `apt-get.is_converged()`
# below (rul24-vouch-is-verdict-authoring, 24A §1c), inert at HEAD (the lift keys only on
# `.predict`). Its OWN refusal is `return 2` — the house style (rul-rc-partition: >=2 = CONFUSED
# ⇒ run), so a refusal can never read as check-passed WHEN GLUED CORRECTLY. The pin forbids a
# build that lets EITHER refuse path (the predict's rc-0, or a mis-glued verdict) read as passed.
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

# THE VOUCH (inert at HEAD; Stage 3 consumes it). CONVERGED sense declared by name
# (rul-role-split); rc-partition: 0 = converged, 1 = diverged, >=2 = confused ⇒ run. The
# REFUSAL path returns 2 (never 0): a second operand, or an unmodeled verb, is CONFUSED — it
# must never masquerade as converged (which a guard would read as "skip the mutator"). This is
# the house `UNK`/`exit 254`-style refusal in verdict-function form (see USER_STORY stage-4).
apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   [ "$2" = "" ] || return 2
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
