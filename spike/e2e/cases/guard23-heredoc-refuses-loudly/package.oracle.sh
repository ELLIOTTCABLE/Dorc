# minimal package oracle (apt/dpkg), lifted statically by dorc — the guard23-* fixture variant.
# The predict() below is the corpus-standard entity-resolver (its stripped body is the PROBE the
# vouched sites ship); the VOUCH is the authored verdict function `apt-get.is_converged()`
# (rul24-vouch-is-verdict-authoring, 24A §1c — authoring it IS the vouch; the two-level bare-mark
# strawman is DEAD). Its stripped body ships as the guard preamble; `( apt_get__is_converged … )
# || <original>` re-checks live at apply (rul-ternary-verdict). [Re-authored by the Stage-3 Part-A
# builder: added the verdict function; conductor-flagged.]
# command-keyed predict(): the oracle's OWN argparse → inline kind-annotation (the real
# entity-resolution; task-W). Flag-strip (pre- and post-verb), bind the verb, annotate
# the single operand as `package`; the `[ "$2" = "" ]` guard refuses a SECOND operand
# (so `install nginx curl` resolves no probe ⇒ runs — no wrong single-entity elision).
apt_get__predict() {
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

# THE VOUCH (rul24-vouch-is-verdict-authoring): sense DECLARED BY NAME (rul-role-split);
# rc-partition 0=converged / 1=diverged / >=2=confused⇒run. The install path runs the same
# dpkg-query the predict does; an unmodeled verb reaches NO arm ⇒ Declined ⇒ no vouch ⇒ run
# (hz-refusepath: an unhandled path never vacuously vouches). The dialect has no `return`, so
# declines are spelled as unhandled paths (tc-verdict-return).
apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
   esac
}
